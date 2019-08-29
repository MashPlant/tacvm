use indexmap::IndexSet;
use crate::{exec::{intrinsic, IntrinsicFn}, MAIN};

#[derive(Debug, Clone)]
pub struct RawProgram<'a> {
  pub vtbl: Vec<RawVTbl<'a>>,
  pub func: Vec<RawFunc<'a>>,
}

#[derive(Debug, Clone)]
pub struct RawVTbl<'a> {
  pub name: &'a str,
  pub line: u32,
  pub data: Vec<RawVTblSlot<'a>>,
}

#[derive(Debug, Clone)]
pub struct RawVTblSlot<'a> {
  pub line: u32,
  pub kind: RawVTblSlotKind<'a>,
}

#[derive(Debug, Clone)]
pub enum RawVTblSlotKind<'a> {
  VTblRef(Option<&'a str>),
  String(Box<str>),
  FuncRef(&'a str),
}

#[derive(Debug, Clone)]
pub struct RawFunc<'a> {
  pub name: &'a str,
  pub line: u32,
  pub code: Vec<RawInst<'a>>,
}

#[derive(Debug, Clone)]
pub struct RawInst<'a> {
  pub line: u32,
  pub kind: RawInstKind<'a>,
}

#[derive(Debug, Clone)]
pub enum RawInstKind<'a> {
  Bin(BinOp, u32, Operand, Operand),
  Un(UnOp, u32, Operand),
  Mv(u32, Operand),
  Param(Operand),
  Call(Option<u32>, CallKind<'a>),
  Ret(Option<Operand>),
  J(u32),
  B(Operand, bool, u32),
  Label(u32),
  Load(u32, u32, i32),
  Store(Operand, u32, i32),
  LStr(u32, Box<str>),
  LVTbl(u32, &'a str),
}

#[derive(Debug, Copy, Clone)]
pub enum Operand {
  Reg(u32),
  Const(i32),
}

#[derive(Debug, Copy, Clone)]
pub enum CallKind<'a> {
  Reg(u32),
  Named(&'a str),
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum BinOp { Add, Sub, Mul, Div, Mod, And, Or, Eq, Ne, Lt, Le, Gt, Ge }

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum UnOp { Neg, Not }

impl BinOp {
  pub fn eval(self, l: i32, r: i32) -> Option<i32> {
    use BinOp::*;
    match self {
      Add => Some(l.wrapping_add(r)),
      Sub => Some(l.wrapping_sub(r)),
      Mul => Some(l.wrapping_mul(r)),
      Div => l.checked_div(r),
      Mod => l.checked_rem(r),
      And => Some(((l != 0) && (r != 0)) as i32),
      Or => Some(((l != 0) || (r != 0)) as i32),
      Eq => Some((l == r) as i32),
      Ne => Some((l != r) as i32),
      Lt => Some((l < r) as i32),
      Le => Some((l <= r) as i32),
      Gt => Some((l > r) as i32),
      Ge => Some((l >= r) as i32),
    }
  }
}

impl UnOp {
  pub fn eval(self, r: i32) -> i32 {
    match self { UnOp::Neg => r.wrapping_neg(), UnOp::Not => (r == 0) as i32, }
  }
}

pub struct Program<'a> {
  pub entry: u32,
  pub vtbl: Vec<VTbl<'a>>,
  pub func: Vec<Func<'a>>,
  pub code: Vec<Inst>,
  pub raw_code: Vec<&'a RawInst<'a>>,
  pub str_pool: IndexSet<&'a str>,
}

pub struct VTbl<'a> {
  pub data: Box<[u32]>,
  pub raw_vtbl: &'a RawVTbl<'a>,
}

pub struct Func<'a> {
  pub entry: u32,
  pub stack_size: u32,
  pub raw_func: &'a RawFunc<'a>,
}

impl<'a> Program<'a> {
  pub fn new(raw: &'a RawProgram<'a>) -> Result<Program<'a>, String> {
    let (mut vtbl_set, mut func_set, mut str_pool) = (IndexSet::new(), IndexSet::new(), IndexSet::<&str>::new());
    for v in &raw.vtbl {
      if !vtbl_set.insert(v.name) {
        return Err(format!("Line {}: duplicate vtbl `{}`.", v.line, v.name));
      }
    }
    for f in &raw.func {
      if !func_set.insert(f.name) {
        return Err(format!("Line {}: duplicate function `{}`.", f.line, f.name));
      }
    }
    let (mut vtbl, mut func, mut code, mut raw_code) = (Vec::new(), Vec::new(), Vec::new(), Vec::new());
    for v in &raw.vtbl {
      let mut data = vec![0; v.data.len()].into_boxed_slice();
      for (idx, s) in v.data.iter().enumerate() {
        data[idx] = match &s.kind {
          &RawVTblSlotKind::VTblRef(v) => if let Some(v) = v {
            if let Some((idx, _)) = vtbl_set.get_full(v) { idx as u32 } else { return Err(format!("Line {}: no such vtbl `{}`.", s.line, v)); }
          } else { 0 },
          RawVTblSlotKind::String(s) => str_pool.insert_full(s).0 as u32,
          &RawVTblSlotKind::FuncRef(f) =>
            if let Some((idx, _)) = func_set.get_full(f) { idx as u32 } else { return Err(format!("Line {}: no such function `{}`.", s.line, f)); }
        }
      }
      vtbl.push(VTbl { data, raw_vtbl: v });
    }
    let mut label_set = Vec::new();
    for f in &raw.func {
      label_set.clear();
      let entry = code.len() as u32;
      let mut idx = entry;
      for i in &f.code {
        if let RawInstKind::Label(l) = i.kind {
          let l = l as usize;
          if label_set.len() <= l {
            label_set.resize(l + 1, 0);
          }
          label_set[l] = idx;
        } else { idx += 1; }
      }
      let mut idx = entry;
      let mut stack_size = 0;
      let mut upd = |r: u32| (stack_size = r.max(stack_size), r).1;
      for i in &f.code {
        use Operand::*;
        use Inst::*;
        let chk_label = |l: u32| label_set.get(l as usize).map(|l| *l).ok_or_else(||
          format!("Line {}: no such label `_L{}` in function `{}`.", i.line, l, f.name));
        let inst = match &i.kind {
          &RawInstKind::Bin(op, d, l, r) => {
            upd(d);
            match (l, r) {
              (Reg(l), Reg(r)) => BinRR(op, d, upd(l), upd(r)),
              (Reg(l), Operand::Const(r)) => BinRC(op, d, upd(l), r),
              (Const(l), Reg(r)) => BinCR(op, d, l, upd(r)),
              (Const(l), Const(r)) =>
                Li(d, if let Some(i) = op.eval(l, r) { i } else { return Err(format!("Line {}: attempt to divide or mod by 0.", i.line)); }),
            }
          }
          &RawInstKind::Un(op, d, r) => match r {
            Reg(r) => match op { UnOp::Neg => Neg(upd(d), upd(r)), UnOp::Not => Not(upd(d), upd(r)) }
            Const(r) => Li(d, op.eval(r)),
          }
          &RawInstKind::Mv(d, r) => match r { Reg(r) => Mv(upd(d), upd(r)), Const(r) => Li(upd(d), r), }
          &RawInstKind::Param(r) => match r { Reg(r) => ParamR(upd(r)), Const(r) => ParamC(r), }
          &RawInstKind::Call(d, c) => {
            if let Some(d) = d { upd(d); }
            match c {
              CallKind::Reg(c) => CallV(d, upd(c)),
              CallKind::Named(name) => if let Some(f) = intrinsic(name) { CallI(d, f) } else {
                if let Some((idx, _)) = func_set.get_full(name) { CallS(d, idx as u32) } else { return Err(format!("Line {}: no such function `{}`.", i.line, name)); }
              }
            }
          }
          &RawInstKind::Ret(r) => match r { None => Ret, Some(Reg(r)) => RetR(upd(r)), Some(Const(r)) => RetC(r), }
          &RawInstKind::J(l) => J(chk_label(l)?),
          &RawInstKind::B(r, z, l) => {
            let l = chk_label(l)?;
            match r {
              Reg(r) => if z { Bz(upd(r), l) } else { Bnz(upd(r), l) },
              Const(r) => J(if z == (r == 0) { l } else { idx }),
            }
          }
          &RawInstKind::Label(_) => continue,
          &RawInstKind::Load(d, base, off) => Load(upd(d), upd(base), off),
          &RawInstKind::Store(r, base, off) => match r { Reg(r) => StoreR(upd(r), base, off), Const(r) => StoreC(r, base, off) }
          RawInstKind::LStr(d, s) => Li(upd(*d), str_pool.insert_full(s).0 as i32),
          &RawInstKind::LVTbl(d, v) => Li(upd(d), if let Some((idx, _)) = vtbl_set.get_full(v) { idx as i32 } else { return Err(format!("Line {}: no such vtbl `{}`.", i.line, v)); })
        };
        idx += 1;
        code.push(inst);
        raw_code.push(i);
      }
      func.push(Func { entry, stack_size, raw_func: f })
    }
    let entry = if let Some((idx, _)) = func_set.get_full(MAIN) { idx as u32 } else {
      return Err(format!("No function named `{}` found.", MAIN));
    };
    Ok(Program { entry, vtbl, func, code, raw_code, str_pool })
  }
}

pub enum Inst {
  BinRR(BinOp, u32, u32, u32),
  BinRC(BinOp, u32, u32, i32),
  BinCR(BinOp, u32, i32, u32),
  Neg(u32, u32),
  Not(u32, u32),
  Mv(u32, u32),
  Li(u32, i32),
  ParamR(u32),
  ParamC(i32),
  CallI(Option<u32>, IntrinsicFn),
  CallS(Option<u32>, u32),
  CallV(Option<u32>, u32),
  Ret,
  RetR(u32),
  RetC(i32),
  J(u32),
  Bz(u32, u32),
  Bnz(u32, u32),
  Load(u32, u32, i32),
  StoreR(u32, u32, i32),
  StoreC(i32, u32, i32),
}
