pub enum MemErr {
  AlignErr,
  NoSuchObj,
  ObjOverflow,
}

pub struct Mem {
  data: Vec<i32>,
  range: Vec<(i32, i32)>,
  str: Vec<Box<str>>,
}

impl Mem {
  pub fn alloc(&mut self, size: u32) -> Result<u32, MemErr> {
    if size % 4 != 0 {
      return Err(MemErr::AlignErr);
    }
    let size = size as usize / 4;
    let old = self.data.len();
    self.data.resize(old + size, 0);
    for i in 0..size {
      self.range.push((-(i as i32), (size - i) as i32));
    }
    Ok(old as u32 * 4)
  }

  pub fn load(&self, base: i32, off: i32) -> Result<i32, MemErr> {
    Ok(self.data[self.check(base, off)?])
  }

  pub fn store(&mut self, base: i32, off: i32, value: i32) -> Result<(), MemErr> {
    let idx = self.check(base, off)?;
    Ok(self.data[idx] = value)
  }

  fn check(&self, base: i32, off: i32) -> Result<usize, MemErr> {
    if base % 4 != 0 || off % 4 != 0 {
      return Err(MemErr::AlignErr);
    }
    let (base, off) = (base / 4, off / 4);
    match self.range.get(base as usize) {
      Some((lb, ub)) => {
        if *lb <= off && off < *ub {
          Ok((base as i32 + off) as usize)
        } else {
          Err(MemErr::ObjOverflow)
        }
      }
      None => Err(MemErr::NoSuchObj), // if base < 0 will reach this branch
    }
  }
}

impl Mem {
  pub fn get_str(&self, idx: i32) -> Option<&str> {
    self.str.get(idx as usize).map(|x| x.as_ref())
  }

  pub fn add_str(&mut self, s: &str) -> i32 {
    self.str.push(s.into());
    self.str.len() as i32 - 1
  }
}