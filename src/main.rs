use cpal;
use cpal::traits::*;

mod audioproccesing;

fn main() {
    let host = cpal::default_host();
    // enumerate all available input formats
    audioproccesing::audioinputs::listinputs();
}
