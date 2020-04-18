#![allow(unused)] // don't warn on unused things
use cpal;
use cpal::traits::*;

mod audioproccesing;

fn main() {
    // enumerate all available input formats
    audioproccesing::audioinputs::list_inputs();
    audioproccesing::audioinputs::list_hosts();
    audioproccesing::audioinputs::list_default_device();
}
