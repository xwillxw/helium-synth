#![allow(unused_imports,unused_variables,dead_code)]
use std::time::Instant;
use rodio::OutputStream;
use startup::{Note, WavetableOscillator, generate_oscillators};
use startup::Filter;
use crate::startup::generate_patch;
mod startup;
mod output;
mod processing {
    pub mod filter_processor;
}

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
    patch.filter.filter_cutoff = 10000;
    
    
    
    notes.push(note_array[50]);
    notes.push(note_array[50]);
    output::play_oscillator(&patch, &notes, 200, &stream_handle);
    notes.clear();
    
    notes.push(note_array[52]);
    notes.push(note_array[50]);
    output::play_oscillator(&patch, &notes, 200, &stream_handle);
    notes.clear();

    notes.push(note_array[53]);
    notes.push(note_array[50]);
    output::play_oscillator(&patch, &notes, 200, &stream_handle);
    notes.clear();

    notes.push(note_array[55]);
    notes.push(note_array[62]);
    output::play_oscillator(&patch, &notes, 200, &stream_handle);
    notes.clear();

    notes.push(note_array[52]);
    notes.push(note_array[59]);
    output::play_oscillator(&patch, &notes, 400, &stream_handle);
    notes.clear();

    notes.push(note_array[48]);
    notes.push(note_array[55]);
    output::play_oscillator(&patch, &notes, 200, &stream_handle);
    notes.clear();

    notes.push(note_array[50]);
    notes.push(note_array[57]);
    output::play_oscillator(&patch, &notes, 400, &stream_handle);
    notes.clear();

   
}