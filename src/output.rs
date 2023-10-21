use std::path;
use std::time::{Instant, Duration};

use rodio::source::SineWave;
//use std::time::Instant;
use rodio::{OutputStream, OutputStreamHandle, Sink};
use rodio::{source::Source, PlayError};
use crate::startup::SynthPatch;
use crate::startup::WavetableOscillator;
use crate::startup::Note;
use crate::processing::filter_processor;

#[inline]
pub fn play_oscillator(patch: &SynthPatch, notes: &Vec<Note>, duration: u64, stream_handle: &OutputStreamHandle) {

    let sink = Sink::try_new(&stream_handle).unwrap();
    let mut global_oscillator = patch.oscillator_type.clone();
    let mut oscillator_stack_index: f32 = 0.0;
    let mut oscillator_stack_index_increment: f32 = 0.0;

    for i in 0..notes.len() {

        let mut oscillator = patch.oscillator_type.clone();
        oscillator.set_frequency(notes[i].frequency);
        oscillator.apply_filter(&patch.filter);
        oscillator_stack_index = oscillator_stack_index + oscillator.index;
        oscillator_stack_index_increment = oscillator_stack_index_increment + oscillator.index;
        
    }
    
    global_oscillator.index = oscillator_stack_index;
    global_oscillator.index_increment = oscillator_stack_index_increment;

    sink.append(global_oscillator.take_duration(Duration::from_millis(duration)));
    sink.sleep_until_end();

    //let now = Instant::now();
    //let elapsed = now.elapsed().as_millis();
    //println!("Elapsed: {:.2?}", elapsed);
}