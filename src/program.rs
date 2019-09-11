use indexmap::IndexSet;
use crate::{vm::{intrinsic, IntrinsicFn}, MAIN};

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
  Int(i32),
  VTblRef(&'a str),
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
  pub code: &'a str,
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

#[derive(Debug, Copy, Clone)]
pub enum BinOp { Add, Sub, Mul, Div, Mod, And, Or, Eq, Ne, Lt, Le, Gt, Ge }

#[derive(Debug, Copy, Clone)]
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
  // entry function index
  pub entry: u32,
  pub vtbl: Vec<Box<[VTblSlot]>>,
  pub func: Vec<Func<'a>>,
  pub str_pool: IndexSet<&'a str>,
}

#[derive(Copy, Clone)]
pub enum VTblSlot {
  Int(i32),
  VTblRef(u32),
  String(u32),
  FuncRef(u32),
}

pub struct Func<'a> {
  pub stack_size: u32,
  pub code: Vec<Inst>,
  pub raw_code: Vec<&'a RawInst<'a>>,
  pub raw_func: &'a RawFunc<'a>,
}

impl<'a> Program<'a> {
  pub fn new(raw: &'a RawProgram<'a>) -> Result<Program<'a>, String> {
    let (mut vtbl_set, mut func_set, mut str_pool) = (IndexSet::new(), IndexSet::new(), IndexSet::<&str>::new());
    for v in &raw.vtbl {
      if !vtbl_set.insert(v.name) {
        return Err(format!("line {}: duplicate vtbl `{}`", v.line, v.name));
      }
    }
    for f in &raw.func {
      if !func_set.insert(f.name) {
        return Err(format!("line {}: duplicate function `{}`", f.line, f.name));
      }
    }
    let mut vtbl = Vec::with_capacity(raw.vtbl.len());
    for v in &raw.vtbl {
      let mut data = Vec::with_capacity(v.data.len());
      for s in &v.data {
        data.push(match &s.kind {
          &RawVTblSlotKind::Int(i) => VTblSlot::Int(i),
          &RawVTblSlotKind::VTblRef(v) => if let Some((idx, _)) = vtbl_set.get_full(v) { VTblSlot::VTblRef(idx as u32) } else {
            return Err(format!("line {}: no such vtbl `{}`", s.line, v));
          }
          RawVTblSlotKind::String(s) => VTblSlot::String(str_pool.insert_full(s).0 as u32),
          &RawVTblSlotKind::FuncRef(f) => if let Some((idx, _)) = func_set.get_full(f) { VTblSlot::FuncRef(idx as u32) } else {
            return Err(format!("line {}: no such function `{}`", s.line, f));
          }
        });
      }
      vtbl.push(data.into());
    }
    let mut func = Vec::with_capacity(raw.func.len());
    let mut label_set = Vec::new();
    for f in &raw.func {
      let (mut code, mut raw_code) = (Vec::new(), Vec::new());
      label_set.clear();
      let mut idx = 0;
      for i in &f.code {
        idx += match &i.kind {
          &RawInstKind::Label(l) => {
            let l = l as usize;
            if label_set.len() <= l {
              label_set.resize(l + 1, 0);
            }
            label_set[l] = idx;
            0
          }
          &RawInstKind::Call(d, _) => if d.is_some() { 2 } else { 1 },
          _ => 1,
        }
      }
      let mut idx = 0;
      let mut max_stack = 0;
      let mut upd = |r: u32| (max_stack = r.max(max_stack), r).1;
      for i in &f.code {
        use Operand::*;
        use Inst::*;
        let chk_label = |l: u32| label_set.get(l as usize).map(|l| *l).ok_or_else(||
          format!("line {}: no such label `_L{}` in function `{}`", i.line, l, f.name));
        let inst = match &i.kind {
          &RawInstKind::Bin(op, d, l, r) => {
            upd(d);
            match (l, r) {
              (Reg(l), Reg(r)) => BinRR(op, d, upd(l), upd(r)),
              (Reg(l), Operand::Const(r)) => BinRC(op, d, upd(l), r),
              (Const(l), Reg(r)) => BinCR(op, d, l, upd(r)),
              (Const(l), Const(r)) =>
                Li(d, if let Some(i) = op.eval(l, r) { i } else { return Err(format!("line {}: attempt to divide or mod by 0", i.line)); }),
            }
          }
          &RawInstKind::Un(op, d, r) => match r {
            Reg(r) => match op { UnOp::Neg => Neg(upd(d), upd(r)), UnOp::Not => Not(upd(d), upd(r)) }
            Const(r) => Li(d, op.eval(r)),
          }
          &RawInstKind::Mv(d, r) => match r { Reg(r) => Mv(upd(d), upd(r)), Const(r) => Li(upd(d), r), }
          &RawInstKind::Param(r) => match r { Reg(r) => ParamR(upd(r)), Const(r) => ParamC(r), }
          &RawInstKind::Call(d, c) => {
            code.push(match c {
              CallKind::Reg(r) => Call(Operand::Reg(upd(r))),
              CallKind::Named(name) => if let Some(i) = intrinsic(name) { Intrinsic(i) } else {
                if let Some((idx, _)) = func_set.get_full(name) { Call(Operand::Const(idx as i32)) } else { return Err(format!("line {}: no such function `{}`", i.line, name)); }
              }
            });
            raw_code.push(i);
            idx += if let Some(d) = d {
              code.push(GetRet(d));
              raw_code.push(i);
              upd(d);
              2
            } else { 1 };
            continue;
          }
          &RawInstKind::Ret(r) => {
            if let Some(Operand::Reg(r)) = r { upd(r); }
            Ret(r)
          }
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
          RawInstKind::LStr(d, s) => LStr(upd(*d), str_pool.insert_full(s).0 as u32),
          &RawInstKind::LVTbl(d, v) => LVTbl(upd(d), if let Some((idx, _)) = vtbl_set.get_full(v) { idx as u32 } else { return Err(format!("line {}: no such vtbl `{}`", i.line, v)); })
        };
        idx += 1; // Call and Label won't reach here
        code.push(inst);
        raw_code.push(i);
      }
      func.push(Func { stack_size: max_stack + 1, code, raw_code, raw_func: f })
    }
    let entry = if let Some((idx, _)) = func_set.get_full(MAIN) { idx as u32 } else {
      return Err(format!("No function named `{}` found", MAIN));
    };
    Ok(Program { entry, vtbl, func, str_pool })
  }
}

// for simple instruction(like Bin, Store), distinguish R and C and handle them separately
// for complex instruction(like Call, Ret), doesn't distinguish R and C, just use Operand
#[derive(Copy, Clone)]
pub enum Inst {
  BinRR(BinOp, u32, u32, u32),
  BinRC(BinOp, u32, u32, i32),
  BinCR(BinOp, u32, i32, u32),
  Neg(u32, u32),
  Not(u32, u32),
  Mv(u32, u32),
  Li(u32, i32),
  LStr(u32, u32),
  LVTbl(u32, u32),
  ParamR(u32),
  ParamC(i32),
  Intrinsic(IntrinsicFn),
  Call(Operand),
  GetRet(u32),
  Ret(Option<Operand>),
  J(u32),
  Bz(u32, u32),
  Bnz(u32, u32),
  Load(u32, u32, i32),
  StoreR(u32, u32, i32),
  StoreC(i32, u32, i32),
}
