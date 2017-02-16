extern crate rustbox;

use visualizer::{Visualizer, Buffer, State};

pub struct Spectrum;

impl Visualizer for Spectrum {
  fn listen(&self, bytes: &Buffer) -> State {
    println!("{:?}", bytes);
    State::Continue
  }
}
