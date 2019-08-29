use tacvm::parser::*;

fn main() {
  let code = include_str!("../tmp.tac");
  match program(Span::new(code)) {
    Ok((_, f)) => for c in f.func {
      println!("{:?}", c);
    }
    Err(e) => eprintln!("{:?}", e),
  }
//  println!("{:?}", inst(Span::new("_T1 = (_T1 + _T1)")));
}