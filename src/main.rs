use tacvm::parser::*;
use tacvm::program::Program;
use tacvm::vm::{VM, RunConfig};

fn main() {
  let code = include_str!("../tmp.tac");
  match program(Span::new(code)) {
    Ok((_, p)) => match Program::new(&p) {
      Ok(p) => {
        let mut vm = VM::new(&p);
        println!("{:?}", vm.run(&mut RunConfig::default_io(100000, !0, true)));
      }
      Err(e) => { println!("{}", e); }
    }
    Err(e) => eprintln!("{:?}", e),
  }
//  println!("{:?}", inst(Span::new("_T1 = (_T1 + _T1)")));
}