use crate::mem::Mem;
use crate::error::Error;
use crate::program::{Program, Inst, VTblSlot, Operand};

pub struct RunConfig {
  pub inst_limit: u32,
  pub stack_limit: u32,
}

pub struct Frame {
  pc: u32,
  func: u32,
  data: Box<[i32]>,
}

impl Frame {
  pub fn new(func: u32, stack_size: u32) -> Frame {
    Frame { pc: !0, func, data: vec![0; stack_size as usize].into_boxed_slice() }// this !0 is useless
  }
}

pub struct VM<'a> {
  mem: Mem,
  stack: Vec<Frame>,
  program: &'a Program<'a>,
  str_addr: Vec<i32>,
  vtbl_addr: Vec<u32>,
}

impl VM<'_> {
  // there shouldn't be any error here, so call unwrap() for Option or Result
  pub fn new<'a>(p: &'a Program<'a>) -> VM<'a> {
    let mut mem = Mem::new();
    let str_addr = p.str_pool.iter().map(|s| mem.define_str(s)).collect::<Vec<_>>();
    let vtbl_addr = p.vtbl.iter().map(|v| mem.alloc(v.len() as u32 * 4).unwrap()).collect::<Vec<_>>();
    for (&addr, v) in vtbl_addr.iter().zip(p.vtbl.iter()) {
      for (idx, &slot) in v.iter().enumerate() {
        let data = match slot {
          VTblSlot::Empty => 0,
          VTblSlot::VTblRef(v) => vtbl_addr[v as usize] as i32,
          VTblSlot::String(s) => str_addr[s as usize],
          VTblSlot::FuncRef(f) => f as i32,
        };
        mem.store(addr, idx as i32 * 4, data).unwrap();
      }
    }
    VM { mem, stack: vec![], program: p, str_addr, vtbl_addr }
  }

  // return Ok(()): `main` returns normally, or cfg.inst_limit is reached
  // return Err(...): any other case
  pub fn run(&mut self, cfg: RunConfig) -> Result<(), Error> {
    let p = self.program;
    let mut func = &p.func[p.entry as usize];
    let mut arg = Vec::new();
    let mut ret = 0;
    let mut pc = 0;
    (self.stack.clear(), self.stack.push(Frame::new(p.entry, func.stack_size)));
    for _ in 0..cfg.inst_limit {
      use Inst::*;
      let frame = self.stack.last_mut().unwrap();
      let stk = frame.data.as_mut();
      let inst = *func.code.get(pc as usize).ok_or(Error::IFOutOfRange)?;
      match inst {
        BinRR(op, d, l, r) => stk[d as usize] = op.eval(stk[l as usize], stk[r as usize]).ok_or(Error::Div0)?,
        BinRC(op, d, l, r) => stk[d as usize] = op.eval(stk[l as usize], r).ok_or(Error::Div0)?,
        BinCR(op, d, l, r) => stk[d as usize] = op.eval(l, stk[r as usize]).ok_or(Error::Div0)?,
        Neg(d, r) => stk[d as usize] = stk[r as usize].wrapping_neg(),
        Not(d, r) => stk[d as usize] = (stk[r as usize] == 0) as i32,
        Mv(d, r) => stk[d as usize] = stk[r as usize],
        Li(d, i) => stk[d as usize] = i,
        LStr(d, s) => stk[d as usize] = self.str_addr[s as usize],
        LVTbl(d, v) => stk[d as usize] = self.vtbl_addr[v as usize] as i32,
        ParamR(r) => arg.push(stk[r as usize]),
        ParamC(c) => arg.push(c),
        Intrinsic(i) => {
          ret = i(*arg.get(0).unwrap_or(&0), *arg.get(1).unwrap_or(&0), &mut self.mem)?;
          arg.clear();
        }
        Call(f) => {
          let f = match f { Operand::Reg(r) => stk[r as usize], Operand::Const(c) => c };
          func = &p.func[f as usize];
          if func.stack_size < arg.len() as u32 { return Err(Error::TooMuchArg); }
          (frame.pc = pc + 1, pc = 0);
          self.stack.push(Frame::new(f as u32, func.stack_size));
          let stk = self.stack.last_mut().unwrap().data.as_mut();
          stk[0..arg.len()].copy_from_slice(&arg);
          arg.clear();
          continue;
        }
        GetRet(d) => {
          stk[d as usize] = ret;
        }
        Ret(r) => {
          if let Some(r) = r {
            ret = match r { Operand::Reg(r) => stk[r as usize], Operand::Const(c) => c };
          }
          self.stack.pop();
          if let Some(frame) = self.stack.last_mut() {
            pc = frame.pc;
            func = &p.func[frame.func as usize];
          } else { return Ok(()); } // `main` returns
          continue;
        }
        J(l) => {
          pc = l;
          continue;
        }
        Bz(r, l) => if stk[r as usize] == 0 {
          pc = l;
          continue;
        }
        Bnz(r, l) => if stk[r as usize] != 0 {
          pc = l;
          continue;
        }
        Load(d, base, off) => stk[d as usize] = self.mem.load(stk[base as usize] as u32, off)?,
        StoreR(r, base, off) => self.mem.store(stk[base as usize] as u32, off, stk[r as usize])?,
        StoreC(c, base, off) => self.mem.store(stk[base as usize] as u32, off, c)?,
      }
      pc += 1;
    }
    Ok(())
  }

  pub fn stacktrace(&self) -> String {
    let _ = self.program;
    unimplemented!()
  }
}

pub type IntrinsicFn = fn(arg0: i32, arg1: i32, mem: &mut Mem) -> Result<i32, Error>;

pub fn intrinsic(name: &str) -> Option<IntrinsicFn> {
  match name {
    "_Alloc" => Some(|size, _, mem| mem.alloc(size as u32).map(|i| i as i32)),
    "_ReadLine" => Some(|_, _, mem| {
      let mut s = String::new();
      std::io::stdin().read_line(&mut s).map_err(|_| Error::IO)?;
      Ok(mem.define_str(&s))
    }),
    "_ReadInt" => Some(|_, _, _| try_read!().map_err(|_| Error::IO)),
    "_StringEqual" => Some(|l, r, mem| Ok((mem.get_str(l)? == mem.get_str(r)?) as i32)),
    "_PrintInt" => Some(|i, _, _| (print!("{}", i), Ok(0)).1),
    "_PrintString" => Some(|s, _, mem| (print!("{}", mem.get_str(s)?), Ok(0)).1),
    "_PrintBool" => Some(|b, _, _| (print!("{}", b != 0), Ok(0)).1),
    "_Halt" => Some(|_, _, _| Err(Error::Halt)),
    _ => None
  }
}