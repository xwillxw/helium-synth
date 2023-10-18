#![allow(unused_variables)]
use startup::{Note, WavetableOscillator};
use processing::filter_processor::Filter;

use crate::startup::generate_patch;
mod startup;
mod output;
mod processing {
    pub mod filter_processor;
}

#[allow(dead_code)]
pub struct SynthPatch {
    oscillator_type: WavetableOscillator,
    filter: Filter,
}

fn main() {

    let patch = generate_patch();
    let note_array = startup::generate_notes();
    let mut notes: Vec<Note> = Vec::new();

    notes.push(note_array[50]);
    notes.push(note_array[54]);
    notes.push(note_array[57]);
    output::play_oscillator(&patch, &notes, 1000);

    notes.clear();
    notes.push(note_array[52]);
    notes.push(note_array[54]);
    notes.push(note_array[57]);
    output::play_oscillator(&patch, &notes, 1000);

}