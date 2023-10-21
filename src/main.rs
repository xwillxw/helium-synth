#![allow(unused_variables)]
use startup::{Note, WavetableOscillator, generate_oscillators};
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
    let (sine, square, saw) = generate_oscillators();
    let mut patch = generate_patch();
    let note_array = startup::generate_notes();
    let mut notes: Vec<Note> = Vec::new();
    patch.oscillator_type = square;
    patch.filter.filter_cutoff = 10000;
    

    notes.push(note_array[24]);
    output::play_oscillator(&patch, &notes, 1000);

    notes.clear();
}