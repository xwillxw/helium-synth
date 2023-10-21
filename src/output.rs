use std::time::{Instant, Duration};

//use std::time::Instant;
use rodio::OutputStream;
use rodio::{source::Source, PlayError};
use crate::startup::SynthPatch;
use crate::startup::WavetableOscillator;
use crate::startup::Note;
use crate::processing::filter_processor;

#[inline]
pub fn play_oscillator(patch: &SynthPatch, notes: &Vec<Note>, duration: u64){
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    for i in 0..notes.len() {
        let mut oscillator = WavetableOscillator::new(patch.oscillator_type.sample_rate.clone(), patch.oscillator_type.wave_table.clone());
        oscillator.set_frequency(notes[i].frequency);
        let oscillator = filter_processor::apply_filter(oscillator, &patch.filter);
        let oscillator = oscillator.convert_samples::<f32>();
        let oscillator = stream_handle.play_raw(oscillator);
        let _result: &Result<(), PlayError> = &oscillator;
    }
    std::thread::sleep(std::time::Duration::from_millis(duration));
    let now = Instant::now();
    drop(stream_handle); drop(_stream);
    let elapsed = now.elapsed().as_millis();
    println!("Elapsed: {:.2?}", elapsed);
    
}