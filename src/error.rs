#[derive(Debug)]
pub enum Error {
  // actually Halt is not an error, but they can be handled in a similar way
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
  // call stack exceeds a given level
  StackOverflow,
  // the number of function arguments > function's stack size
  TooMuchArg,
  // div 0 or mod 0
  Div0,
  // now only reading things from console will trigger this
  IO,
}