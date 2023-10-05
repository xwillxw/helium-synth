#![ allow(unused_variables)]
use std::vec::Vec;
use startup::Note;
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
    let mut notes: Vec<Note> = Vec::new();
    notes.push(note_array[50]);
    notes.push(note_array[54]);
    notes.push(note_array[57]);
    output::play_oscillator(&square, notes[1]);
}