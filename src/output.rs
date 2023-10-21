use std::io::sink;
use std::path;
use std::time::{Instant, Duration};
use rodio::buffer::SamplesBuffer;
use rodio::source::SineWave;
//use std::time::Instant;
use rodio::{OutputStream, OutputStreamHandle, Sink, buffer};
use rodio::{source::Source, PlayError};
use crate::processing::filter_processor;
use crate::startup::SynthPatch;
use crate::startup::WavetableOscillator;
use crate::startup::Note;

#[inline]
pub fn play_oscillator(patch: &SynthPatch, notes: &Vec<Note>, duration: u64, stream_handle: &OutputStreamHandle) {

    let mut sink_stack: Vec<Sink> = Vec::new();
    let buffer = SamplesBuffer::new(1, 44100, vec![1i16, 2, 3, 4, 5, 6]);
    
    for i in 0..notes.len() {


        

        let mut oscillator_voicing_1 = patch.oscillator_type.clone();
        let mut oscillator_voicing_2 = patch.oscillator_type.clone();
        let mut oscillator_voicing_3 = patch.oscillator_type.clone();
        let mut oscillator_voicing_4 = patch.oscillator_type.clone();

        oscillator_voicing_1.set_frequency(notes[i].frequency + 4.0);
        let oscillator_voicing_1 = filter_processor::apply_filter(&oscillator_voicing_1, &patch.filter);
        oscillator_voicing_2.set_frequency(notes[i].frequency + 2.0);
        let oscillator_voicing_2 = filter_processor::apply_filter(&oscillator_voicing_2, &patch.filter);
        oscillator_voicing_3.set_frequency(notes[i].frequency - 2.0);
        let oscillator_voicing_3 = filter_processor::apply_filter(&oscillator_voicing_3, &patch.filter);
        oscillator_voicing_4.set_frequency(notes[i].frequency - 4.0);
        let oscillator_voicing_3 = filter_processor::apply_filter(&oscillator_voicing_4, &patch.filter);
        

        let sink = Sink::try_new(&stream_handle).unwrap();
        sink.append(oscillator_voicing_1.take_duration(Duration::from_millis(duration)).amplify(0.2));
        sink_stack.push(sink);

        let sink = Sink::try_new(&stream_handle).unwrap();
        sink.append(oscillator_voicing_2.take_duration(Duration::from_millis(duration)).amplify(0.2));
        sink_stack.push(sink);

        let sink = Sink::try_new(&stream_handle).unwrap();
        sink.append(oscillator_voicing_3.take_duration(Duration::from_millis(duration)).amplify(0.2));
        sink_stack.push(sink);

        let sink = Sink::try_new(&stream_handle).unwrap();
        sink.append(oscillator_voicing_4.take_duration(Duration::from_millis(duration)).amplify(0.2));
        sink_stack.push(sink);
    }
    
    for i in 0..sink_stack.len() {
        sink_stack[i].sleep_until_end();
    }
}