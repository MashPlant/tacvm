mod util;
pub mod parser;
pub mod program;
pub mod mem;
pub mod error;
pub mod vm;

pub const MAIN: &str = "main";

// some string keywords in parser
pub const VTBL: &str = "VTBL";
pub const FUNC: &str = "FUNC";
pub const PARAM: &str = "parm";
pub const CALL: &str = "call";
pub const RETURN: &str = "return";
pub const BRANCH: &str = "branch";
pub const ID_PREFIX: &str = "%";

// the id of str / func starts growing from *_OFFSET; we won't depend on it, it is only used for debugging
pub const STR_OFFSET: i32 = 0xA0000000u32 as i32;
pub const FUNC_OFFSET: i32 = 0xB0000000u32 as i32;
// the initial value of virtual registers
pub const UNINITIALIZED: i32 = 0xDDDDDDDDu32 as i32;

use std::io::{self, BufRead, Write};

pub fn work(code: &str, inst_limit: u32, stack_limit: u32, stacktrace: bool, inst_count: bool, vm_input: Box<dyn BufRead>, vm_output: Box<dyn Write>, info_output: Box<dyn Write>) -> io::Result<()> {
  let mut cfg = vm::RunConfig { inst_limit, stack_limit, stacktrace, inst_count, vm_input, vm_output, info_output };
  match parser::program(parser::Span::new(&code)) {
    Ok((_, p)) => match program::Program::new(&p) {
      Ok(p) => vm::VM::new(&p).run(&mut cfg)?,
      Err(e) => writeln!(cfg.info_output, "Parser error: {}.", e)?,
    }
    Err(e) => match e {
      nom::Err::Error((span, _)) | nom::Err::Failure((span, _)) =>
        writeln!(cfg.info_output, "Parser error: syntax error at {}:{}.", span.line, span.get_column())?,
      nom::Err::Incomplete(_) => unreachable!(), // we didn't use nom's stream mode, won't have Incomplete
    }
  }
  Ok(())
}