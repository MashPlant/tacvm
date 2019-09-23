#[derive(Debug)]
pub enum Error {
  // program calls _Halt explicitly
  Halt,
  // base % 4 != 0 or off % 4 != 0 or allocate size % 4 != 0
  UnalignedMem,
  // base == 0
  NullPointer,
  // base is in an invalid object
  MemOutOfRange,
  // base is in a valid obj, base + off is not or in another obj
  ObjOutOfRange,
  // access to string id is invalid(typically a string id is not a valid mem address)
  StrOutOfRange,
  // instruction fetch out of range
  IFOutOfRange,
  // call a register which is not a valid function id
  CallOutOfRange,
  // call stack exceeds a given level(specified in RunConfig)
  StackOverflow,
  // instructions exceeds a given number(specified in RunConfig)
  TLE,
  // the number of function arguments > function's stack size
  TooMuchArg,
  // div 0 or mod 0
  Div0,
  // fails in reading or writing things
  IO,
}