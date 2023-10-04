#![ allow(unused_variables)]

mod startup;
mod output;
mod processing {
    pub mod filter;
}

fn main() {

    //Generating array of notes, names and frequencies
    let note_array = startup::generate_notes();

    //Generating oscillator types
    let (sine,square,saw) = startup::generate_oscillators();
    
    output::play_oscillator(&square, note_array[14]);
}