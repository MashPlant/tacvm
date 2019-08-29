use crate::error::Error;

pub struct Mem {
  data: Vec<i32>,
  range: Vec<(i32, i32)>,
  str: Vec<Box<str>>,
}

impl Mem {
  // object address are u32, and must be positive i32(ignore overflow), starting from 4 and increase by 4
  // string id are negative i32(ignore overflow), starting from -1 and decrease by 1
  pub fn new() -> Mem {
    Mem { data: vec![0], range: vec![(0, 0)], str: vec![] }
  }

  pub fn alloc(&mut self, size: u32) -> Result<u32, Error> {
    if size % 4 != 0 { return Err(Error::UnalignedMem); }
    let size = size as usize / 4;
    let old = self.data.len();
    self.data.resize(old + size, 0);
    for i in 0..size {
      self.range.push((-(i as i32), (size - i) as i32));
    }
    Ok(old as u32 * 4)
  }

  pub fn load(&self, base: u32, off: i32) -> Result<i32, Error> {
    Ok(self.data[self.check(base, off)?])
  }

  pub fn store(&mut self, base: u32, off: i32, value: i32) -> Result<(), Error> {
    let idx = self.check(base, off)?;
    Ok(self.data[idx] = value)
  }

  fn check(&self, base: u32, off: i32) -> Result<usize, Error> {
    if base == 0 { return Err(Error::NullPointer); }
    if base % 4 != 0 || off % 4 != 0 { return Err(Error::UnalignedMem); }
    let (base, off) = (base / 4, off / 4);
    match self.range.get(base as usize) {
      Some((lb, ub)) => {
        if *lb <= off && off < *ub {
          Ok((base as i32 + off) as usize)
        } else {
          Err(Error::ObjOutOfRange)
        }
      }
      None => Err(Error::MemOutOfRange), // if base < 0 will reach this branch
    }
  }
}

impl Mem {
  pub fn define_str(&mut self, s: &str) -> i32 {
    self.str.push(s.into());
    -(self.str.len() as i32)
  }

  pub fn get_str(&self, idx: i32) -> Result<&str, Error> {
    self.str.get((-idx - 1) as usize).map(|x| x.as_ref()).ok_or(Error::StrOutOfRange)
  }
}