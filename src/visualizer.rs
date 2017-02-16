#![allow(dead_code)]
use std::io;
use std::ops::Deref;
use std::io::prelude::*;
use std::fs::File;
use std::mem;

pub enum State {
  Finish,
  Continue,
  Error(io::Error)
}

pub type Buffer = Vec<i16>;

pub trait Visualizer {
  fn listen(&self, &mut Buffer) -> State;
}

#[derive(Debug)]
pub struct Preferences {
  pub fifo: String,
  pub rate: usize
}


pub struct Runner<T: Visualizer> {
  pub opts: Preferences,
  viz: Box<T>
}

impl<T: Visualizer> Runner<T> {
  pub fn new(opts: Preferences, viz: T) -> Runner<T> {
    Runner::<T> {
      opts: opts,
      viz: Box::new(viz)
    }
  }

  fn read(&self, n: usize) -> io::Result<Buffer> {
    let fifo = try!(File::open(&self.opts.fifo));
    // read n * 2 bytes as we'll be producing i16s
    let bytebuf: Vec<u8> = fifo.bytes().take(n * 2).map(|b| b.expect("failed to read byte!")).collect();

    Ok(bytebuf.chunks(2).map(|byteslice| {
      unsafe {
        mem::transmute::<[u8; 2], i16>([byteslice[0], byteslice[1]])
      }
    }).collect())
  }

  pub fn run(&self) -> io::Result<()> {
    loop {
      let mut bytebuf: Buffer = try!(self.read(128));

      match self.viz.deref().listen(&mut bytebuf) {
        State::Continue => {},
        State::Finish => break,
        State::Error(err) => {
          return Err(err)
        }
      }
    }

    Ok(())
  }
}
