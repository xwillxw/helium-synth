#![allow(unused_imports,unused_variables,dead_code)]
use std::time::Instant;
use rodio::OutputStream;
use startup::{Note, WavetableOscillator, generate_oscillators, Filter, FilterType};
use crate::startup::generate_patch;
use output::play_oscillator;

mod startup;
mod output;

pub struct SynthPatch {
    oscillator_type: WavetableOscillator,
    filter: Filter,
}

fn main() {
    //let now = Instant::now();
    //println!("Elapsed: {:.2?}", elapsed);
    //notes.clear();
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    
    let (sine, square, saw) = generate_oscillators();
    let mut patch = generate_patch();
    let note_array = startup::generate_notes();
    let mut notes: Vec<Note> = Vec::new();
    patch.oscillator_type = saw;
    patch.filter.filter_type = FilterType::LP;
    patch.filter.filter_cutoff = 20000;

    // notes.push(note_array[46]);
    // output::play_oscillator(&patch, &notes, 400, &stream_handle);
    // notes.clear();

    // this sucks
    // make it better and add sequencer please
    // also use midi moron

    notes.push(note_array[58]);
    play_oscillator(&patch, &notes, 200, &stream_handle);
    notes.clear();
    notes.push(note_array[54]);
    play_oscillator(&patch, &notes, 200, &stream_handle);
    notes.clear();
    notes.push(note_array[58]);
    play_oscillator(&patch, &notes, 200, &stream_handle);
    notes.clear();
    notes.push(note_array[60]);
    play_oscillator(&patch, &notes, 200, &stream_handle);
    notes.clear();
    notes.push(note_array[61]);
    play_oscillator(&patch, &notes, 200, &stream_handle);
    notes.clear();
    notes.push(note_array[60]);
    play_oscillator(&patch, &notes, 200, &stream_handle);
    notes.clear();
    notes.push(note_array[58]);
    play_oscillator(&patch, &notes, 200, &stream_handle);
    notes.clear();
    notes.push(note_array[54]);
    play_oscillator(&patch, &notes, 200, &stream_handle);
    notes.clear();
    notes.push(note_array[53]);
    play_oscillator(&patch, &notes, 400, &stream_handle);
    notes.clear();

}