use std::{io::{Read, ErrorKind}, slice, str};

// I never knew it is so DAMN hard to read an int in rust
// if anyone knows an equivalent implementation using stdlib or another lib, pls tell me
// "equivalent" means: read int from an io::Read, omit leading spaces, stops at io error or spaces

pub trait ReadHelper: Read {
  // this is basically copied from io::Bytes::next(), can't directly use it because bytes() moves self
  fn next_byte(&mut self) -> Option<u8> {
    let mut byte = 0;
    loop {
      return match self.read(slice::from_mut(&mut byte)) {
        Ok(0) => None,
        Ok(..) => Some(byte),
        Err(ref e) if e.kind() == ErrorKind::Interrupted => continue,
        Err(_) => None,
      };
    }
  }

  fn next_int(&mut self) -> Option<i32> {
    let mut buf = Vec::with_capacity(16);
    loop {
      match self.next_byte() {
        Some(b) => if !(b as char).is_ascii_whitespace() {
          buf.push(b);
          break;
        }
        None => return None,
      }
    }
    loop {
      match self.next_byte() {
        Some(b)  if !(b as char).is_ascii_whitespace() => { buf.push(b); }
        _ => return str::from_utf8(&buf).ok()?.parse().ok(),
      }
    }
  }
}

impl<R: Read> ReadHelper for R {}