#![allow(unused_imports,unused_variables,dead_code)]
use std::time::Instant;
use rodio::OutputStream;
use startup::{Note, WavetableOscillator, generate_oscillators, Filter, FilterType};
use crate::startup::generate_patch;
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
    patch.oscillator_type = square;
    patch.filter.filter_type = FilterType::LP;
    patch.filter.filter_cutoff = 100;

    notes.push(note_array[45]);
    notes.push(note_array[46]);
    output::play_oscillator(&patch, &notes, 600, &stream_handle);
    notes.clear();
    notes.push(note_array[46]);
    output::play_oscillator(&patch, &notes, 600, &stream_handle);
    notes.clear();
    notes.push(note_array[48]);
    notes.push(note_array[46]);
    output::play_oscillator(&patch, &notes, 600, &stream_handle);
    notes.clear();
    notes.push(note_array[46]);
    output::play_oscillator(&patch, &notes, 300, &stream_handle);
    notes.clear();
    notes.push(note_array[45]);
    output::play_oscillator(&patch, &notes, 300, &stream_handle);
    notes.clear();
    notes.push(note_array[50]);
    notes.push(note_array[46]);
    output::play_oscillator(&patch, &notes, 300, &stream_handle);
    notes.clear();
    notes.push(note_array[48]);
    output::play_oscillator(&patch, &notes, 300, &stream_handle);
    notes.clear();
    notes.push(note_array[46]);
    output::play_oscillator(&patch, &notes, 600, &stream_handle);
    notes.clear();
    notes.push(note_array[45]);
    notes.push(note_array[46]);
    output::play_oscillator(&patch, &notes, 600, &stream_handle);
    notes.clear();
    notes.push(note_array[46]);
    output::play_oscillator(&patch, &notes, 300, &stream_handle);
    notes.clear();
    notes.push(note_array[48]);
    output::play_oscillator(&patch, &notes, 300, &stream_handle);
    notes.clear();
}