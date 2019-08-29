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
pub enum RawVTblSlot<'a> {
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

#[derive(Debug, Clone)]
pub enum Operand {
  Reg(u32),
  Const(i32),
}

#[derive(Debug, Clone)]
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

pub enum Inst {
  BinRR(BinOp, u32, u32, u32),
  BinRI(BinOp, u32, u32, i32),
  BinIR(BinOp, u32, i32, u32),
  Neg(u32, u32),
  Not(u32, u32),
  Mv(u32, u32),
  Li(u32, i32),
  Load(u32, u32, i32),
  Store(u32, u32, i32),
  J(u32),
  Bz(u32, u32),
  Bnz(u32, u32),
}
