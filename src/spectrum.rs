extern crate termbox;
extern crate rustfft;
extern crate num;

use visualizer::{Visualizer, Buffer, State};
use self::num::Complex;
use self::termbox::*;

pub struct Spectrum {
  termbox: Termbox
} 

impl Spectrum {
  pub fn new() -> Spectrum {
    Spectrum {
      termbox: Termbox::open().expect("Failed to open Termbox!")
    }
  }

  pub fn draw(&mut self, values: &Vec<f64>) {
    let width = self.termbox.width();
    let height = self.termbox.height();

    for x in 0..80 {
      let bar_height = values[x as usize] as i32 % (height / 2);

      for y in 0..bar_height {
        self.termbox.change_cell(x as i32, (height / 2) + y, '▊', attributes::WHITE, attributes::BLACK);
        self.termbox.change_cell(x as i32, (height / 2) - y, '▊', attributes::WHITE, attributes::BLACK);
      }
    }

    self.termbox.present();
    self.termbox.clear();
  }
}

impl Visualizer for Spectrum {
  fn listen(&mut self, bytes: &Buffer) -> State {
    match self.termbox.peek_event(40) {
      Some(Event::Key(event)) => {
        if event.key == KEY_ESC || event.ch == Some('q') {
          return State::Finish
        }
      },

      _ => {}
    }

    /// convert the byte buffer to a vector of Complex<f64>
    let floats: Vec<Complex<f64>> = bytes.iter().map(|b| {
      Complex::new(*b as f64, 0.0)
    }).collect();

    /// take a copy of floats that will be mutated
    let mut transformed = floats.clone();
    let mut fft = self::rustfft::FFT::new(floats.len(), false);

    /// apply fft to the floats
    fft.process(&floats, &mut transformed);

    /// decomplex the floats, square and square-root them then scale
    let squared: Vec<f64> = transformed.iter().map(|cmplx| {
      let n: f64 = cmplx.re + cmplx.im;
      (n * n).sqrt() * 0.00002
    }).collect();

    self.draw(&squared);

    State::Continue
  }
}
