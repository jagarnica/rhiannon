#![allow(unused)] // don't warn on unused things
use cpal;
use cpal::traits::*;

mod audioproccesing;

fn main() {
    // enumerate all available input formats
    audioproccesing::inputs::list_inputs();
    audioproccesing::inputs::list_hosts();
}
