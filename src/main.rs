#![allow(unused)] // don't warn on unused things
use cpal;
use cpal::traits::*;

mod audioproccesing;

fn main() {
    // enumerate all available input formats
    // audioproccesing::inputs::enumerate_device_info();
    //audioproccesing::inputs::list_default_input_formats();
    audioproccesing::inputs::list_available_devices();

    audioproccesing::inputs::list_default_device();

    audioproccesing::inputs::list_hosts();

   
}
