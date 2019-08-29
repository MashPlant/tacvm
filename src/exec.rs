use crate::mem::Mem;
use crate::program::Inst;

pub struct Frame {
  saved_pc: u32,
  data: Box<[u32]>,
}

pub struct VM {
  mem: Mem,
  stack: Vec<Frame>,
}

impl VM {
//  pub fn exec(&mut self, code: &[Inst], limit: u32) {
//    let mut pc = 0;
//    for _ in 0..limit {
//
//    }
//  }
}