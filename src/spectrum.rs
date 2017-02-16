extern crate rustbox;
extern crate rustfft;
extern crate num;

use visualizer::{Visualizer, Buffer, State};
use self::num::Complex;

pub struct Spectrum;

impl Visualizer for Spectrum {
  fn listen(&self, bytes: &Buffer) -> State {
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
      (n * n).sqrt() * 0.0008
    }).collect();

    State::Continue
  }
}
