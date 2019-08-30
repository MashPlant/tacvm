use tacvm::{parser::{program, Span}, program::Program, vm::{VM, RunConfig}};
use clap::{Arg, App};
use std::{fs::{self, File}, io::{self, BufReader, BufRead, Write}};

fn main() -> Result<(), Box<dyn std::error::Error>> {
  let matches = App::new("tacvm").author("MashPlant").about("A naive virtual machine to execute tac code, for compiler course's use.")
    .arg(Arg::with_name("file").required(true))
    .arg(Arg::with_name("inst_limit").long("inst_limit").takes_value(true))
    .arg(Arg::with_name("stack_limit").long("stack_limit").takes_value(true))
    .arg(Arg::with_name("print_stacktrace").long("print_stacktrace"))
    .arg(Arg::with_name("vm_input").long("vm_input").takes_value(true))
    .arg(Arg::with_name("vm_output").long("vm_output").takes_value(true))
    .arg(Arg::with_name("info_output").long("info_output").takes_value(true))
    .get_matches();
  let file = matches.value_of("file").unwrap();
  let code = fs::read_to_string(file)?;
  let inst_limit = matches.value_of("inst_limit").and_then(|s| s.parse().ok()).unwrap_or(!0);
  let stack_limit = matches.value_of("stack_limit").and_then(|s| s.parse().ok()).unwrap_or(!0);
  let print_stacktrace = matches.is_present("print_stacktrace");
  let vm_input = matches.value_of("vm_input")
    .and_then::<Box<dyn BufRead>, _>(|s| Some(Box::new(BufReader::new(File::open(s).ok()?))))
    .unwrap_or_else(|| Box::new(BufReader::new(io::stdin())));
  let vm_output = matches.value_of("vm_output")
    .and_then::<Box<dyn Write>, _>(|s| Some(Box::new(File::create(s).ok()?)))
    .unwrap_or_else(|| Box::new(io::stdout()));
  let mut info_output = matches.value_of("info_output")
    .and_then::<Box<dyn Write>, _>(|s| Some(Box::new(File::create(s).ok()?)))
    .unwrap_or_else(|| Box::new(io::stderr()));
  match program(Span::new(&code)) {
    Ok((_, p)) => match Program::new(&p) {
      Ok(p) => {
        let mut cfg = RunConfig { inst_limit, stack_limit, print_stacktrace, vm_input, vm_output, info_output };
        VM::new(&p).run(&mut cfg)?;
      }
      Err(e) => writeln!(info_output, "Parser error: {}.", e)?,
    }
    Err(e) => match e {
      nom::Err::Error((span, _)) | nom::Err::Failure((span, _)) =>
        writeln!(info_output, "Parser error: syntax error at {}:{}.", span.line, span.get_column())?,
      nom::Err::Incomplete(_) => unreachable!(), // we didn't use nom's stream mode, won't have Incomplete
    }
  }
  Ok(())
}