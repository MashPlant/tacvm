use std::io::{self, Write, BufRead, BufReader};
use crate::mem::Mem;
use crate::error::Error;
use crate::program::{Program, Inst, VTblSlot, Operand};
use crate::util::ReadHelper;

pub struct RunConfig {
  pub inst_limit: u32,
  pub stack_limit: u32,
  // if vm halted with an error && stacktrace == true, stacktrace will be printed
  pub stacktrace: bool,
  pub inst_count: bool,
  pub vm_input: Box<dyn BufRead>,
  pub vm_output: Box<dyn Write>,
  pub info_output: Box<dyn Write>,
}

impl RunConfig {
  pub fn simple() -> Self {
    Self::default_io(!0, !0, false, false)
  }

  pub fn default_io(inst_limit: u32, stack_limit: u32, stacktrace: bool, inst_count: bool) -> Self {
    Self { inst_limit, stack_limit, stacktrace, inst_count, vm_input: Box::new(BufReader::new(io::stdin())), vm_output: Box::new(io::stdout()), info_output: Box::new(io::stderr()) }
  }
}

pub struct Frame {
  pc: u32,
  func: u32,
  data: Box<[i32]>,
}

impl Frame {
  pub fn new(func: u32, stack_size: u32) -> Frame {
    Frame { pc: 0, func, data: vec![0; stack_size as usize].into_boxed_slice() } // this 0 is useless
  }
}

pub struct VM<'a> {
  pc: u32,
  inst_count: u32,
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
          VTblSlot::Int(i) => i,
          VTblSlot::VTblRef(v) => vtbl_addr[v as usize] as i32,
          VTblSlot::String(s) => str_addr[s as usize],
          VTblSlot::FuncRef(f) => f as i32,
        };
        mem.store(addr, idx as i32 * 4, data).unwrap();
      }
    }
    VM { pc: 0, inst_count: 0, mem, stack: vec![], program: p, str_addr, vtbl_addr }
  }

  pub fn run(&mut self, cfg: &mut RunConfig) -> io::Result<()> {
    if let Err(err) = self.run_impl(cfg) {
      writeln!(cfg.info_output, "VM halted with error: {:?}", err)?;
      if cfg.stacktrace {
        writeln!(cfg.info_output, "stacktrace: ")?;
        self.stacktrace(&mut cfg.info_output)?;
      }
    }
    if cfg.inst_count {
      writeln!(cfg.info_output, "Totally {} instruction(s) executed", self.inst_count)?;
    }
    Ok(())
  }

  // return Ok(()): `main` returns normally
  // return Err(...): any other case
  fn run_impl(&mut self, cfg: &mut RunConfig) -> Result<(), Error> {
    let p = self.program;
    let mut func = &p.func[p.entry as usize];
    let mut arg = Vec::new();
    let mut ret = 0; // like the %eax or $v0 register
    (self.pc = 0, self.inst_count = 0);
    (self.stack.clear(), self.stack.push(Frame::new(p.entry, func.stack_size)));
    for _ in 0..cfg.inst_limit {
      use Inst::*;
      let frame = self.stack.last_mut().unwrap();
      let stk = frame.data.as_mut();
      let inst = *func.code.get(self.pc as usize).ok_or(Error::IFOutOfRange)?;
      (self.pc += 1, self.inst_count += 1);
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
          ret = i(*arg.get(0).unwrap_or(&0), *arg.get(1).unwrap_or(&0), &mut self.mem, cfg)?;
          arg.clear();
        }
        Call(f) => {
          let f = match f { Operand::Reg(r) => stk[r as usize], Operand::Const(c) => c };
          func = &p.func[f as usize];
          if func.stack_size < arg.len() as u32 { return Err(Error::TooMuchArg); }
          (frame.pc = self.pc, self.pc = 0);
          if cfg.stack_limit < self.stack.len() as u32 { return Err(Error::StackOverflow); }
          self.stack.push(Frame::new(f as u32, func.stack_size));
          let stk = self.stack.last_mut().unwrap().data.as_mut();
          stk[0..arg.len()].copy_from_slice(&arg);
          arg.clear();
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
            self.pc = frame.pc;
            func = &p.func[frame.func as usize];
          } else { return Ok(()); } // `main` returns
        }
        J(l) => self.pc = l,
        Bz(r, l) => if stk[r as usize] == 0 { self.pc = l; }
        Bnz(r, l) => if stk[r as usize] != 0 { self.pc = l; }
        Load(d, base, off) => stk[d as usize] = self.mem.load(stk[base as usize] as u32, off)?,
        StoreR(r, base, off) => self.mem.store(stk[base as usize] as u32, off, stk[r as usize])?,
        StoreC(c, base, off) => self.mem.store(stk[base as usize] as u32, off, c)?,
      }
    }
    Err(Error::TLE)
  }

  pub fn stacktrace(&self, wt: &mut impl Write) -> io::Result<()> {
    for (idx, f) in self.stack.iter().enumerate() {
      let func = &self.program.func[f.func as usize];
      write!(wt, "  - function `{}`, ", func.raw_func.name)?;
      let pc = if idx + 1 == self.stack.len() { self.pc } else { f.pc } - 1;
      let raw = func.raw_code[pc as usize];
      writeln!(wt, "line {}, code `{}`", raw.line, raw.code)?;
    }
    Ok(())
  }
}

pub type IntrinsicFn = fn(arg0: i32, arg1: i32, mem: &mut Mem, cfg: &mut RunConfig) -> Result<i32, Error>;

pub fn intrinsic(name: &str) -> Option<IntrinsicFn> {
  match name {
    "_Alloc" => Some(|size, _, mem, _| mem.alloc(size as u32).map(|i| i as i32)),
    "_ReadLine" => Some(|_, _, mem, cfg| {
      let mut s = String::new();
      cfg.vm_input.read_line(&mut s).map_err(|_| Error::IO)?;
      Ok(mem.define_str(s.trim_end_matches('\n')))
    }),
    "_ReadInt" => Some(|_, _, _, cfg| Ok(cfg.vm_input.next_int().ok_or(Error::IO)?)),
    "_StringEqual" => Some(|l, r, mem, _| Ok((mem.get_str(l)? == mem.get_str(r)?) as i32)),
    "_PrintInt" => Some(|i, _, _, cfg| {
      write!(cfg.vm_output, "{}", i).map_err(|_| Error::IO)?;
      Ok(0)
    }),
    "_PrintString" => Some(|s, _, mem, cfg| {
      write!(cfg.vm_output, "{}", mem.get_str(s)?).map_err(|_| Error::IO)?;
      Ok(0)
    }),
    "_PrintBool" => Some(|b, _, _, cfg| {
      write!(cfg.vm_output, "{}", b != 0).map_err(|_| Error::IO)?;
      Ok(0)
    }),
    "_Halt" => Some(|_, _, _, _| Err(Error::Halt)),
    _ => None
  }
}