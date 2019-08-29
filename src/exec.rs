use crate::mem::Mem;

pub struct Frame {
  pub saved_pc: u32,
  pub data: Box<[u32]>,
}

pub struct VM {
  pub mem: Mem,
  pub stack: Vec<Frame>,
}

impl VM {
//  pub fn exec(&mut self, code: &[Inst], limit: u32) {
//    let mut pc = 0;
//    for _ in 0..limit {
//
//    }
//  }
}

// intrinsic functions return -1 for halt
const HALT_CODE: u32 = !0;

pub type IntrinsicFn = fn(arg0: u32, arg1: u32, mem: &Mem) -> u32;

pub fn intrinsic(name: &str) -> Option<IntrinsicFn> {
  match name {
    "_Alloc" => Some(|_, _, _| 0),
    "_ReadLine" => Some(|_, _, _| 0),
    "_ReadInt" => Some(|_, _, _| 0),
    "_StringEqual" => Some(|_, _, _| 0),
    "_PrintInt" => Some(|i, _, _| (print!("{:?}", i as i32), 0).1),
    "_PrintString" => Some(|_, _, _| 0),
    "_PrintBool" => Some(|b, _, _| (print!("{:?}", b != 0), 0).1),
    "_Halt" => Some(|_, _, _| HALT_CODE),
    _ => None
  }
}