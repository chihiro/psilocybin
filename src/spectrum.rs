extern crate rustbox;
extern crate dft;

use visualizer::{Visualizer, Buffer, State};
use self::dft::{Operation, Plan};

pub struct Spectrum;

impl Visualizer for Spectrum {
  fn listen(&self, bytes: &mut Buffer) -> State {
    println!("{:?}", bytes);
    State::Continue
  }
}
