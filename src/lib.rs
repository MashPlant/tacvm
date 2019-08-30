mod util;
pub mod parser;
pub mod program;
pub mod mem;
pub mod error;
pub mod vm;

pub const MAIN: &str = "main";

// some string keywords in parser
pub const EMPTY: &str = "<empty>";
pub const VTBL: &str = "VTBL";
pub const FUNCTION: &str = "FUNCTION";
pub const PARAM: &str = "parm";
pub const CALL: &str = "call";
pub const RETURN: &str = "return";
pub const BRANCH: &str = "branch";
pub const REG_PREFIX: &str = "_T";
pub const LABEL_PREFIX: &str = "_L";