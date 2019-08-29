use tacvm::parser::*;
use tacvm::program::Program;

fn main() {
  let code = include_str!("../tmp.tac");
  match program(Span::new(code)) {
    Ok((_, p)) => match Program::new(&p) {
      Ok(_) => {}
      Err(e) => { println!("{}", e); }
    }
    Err(e) => eprintln!("{:?}", e),
  }
//  println!("{:?}", inst(Span::new("_T1 = (_T1 + _T1)")));
}