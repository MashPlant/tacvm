use tacvm::parser::*;
use tacvm::program::Program;
use tacvm::vm::{VM, RunConfig};

fn main() {
  let code = include_str!("../tmp.tac");
  match program(Span::new(code)) {
    Ok((_, p)) => match Program::new(&p) {
      Ok(p) => {
        println!("{:?}", VM::new(&p).run(RunConfig { inst_limit: 100000, stack_limit: !0 }))
      }
      Err(e) => { println!("{}", e); }
    }
    Err(e) => eprintln!("{:?}", e),
  }
//  println!("{:?}", inst(Span::new("_T1 = (_T1 + _T1)")));
}