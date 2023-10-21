use rodio::{OutputStream, source::Source, PlayError};
use crate::startup::WavetableOscillator;
use crate::startup::Note;
use crate::processing::filter_processor;
use crate::processing::filter_processor::Filter;

pub fn play_oscillator(oscillator_type: &WavetableOscillator, notes: &Vec<Note>, duration: u64, current_filter: &Filter){

    let (_stream, stream_handle) = OutputStream::try_default().unwrap();

    for i in 0..notes.len() {
        let mut oscillator = WavetableOscillator::new(oscillator_type.sample_rate.clone(), oscillator_type.wave_table.clone());
        oscillator.set_frequency(notes[i].frequency);
        let oscillator = filter_processor::apply_filter(oscillator, current_filter);
        let oscillator = oscillator.convert_samples::<f32>();
        let oscillator = stream_handle.play_raw(oscillator);
        let _result: &Result<(), PlayError> = &oscillator;
    }

    std::thread::sleep(std::time::Duration::from_millis(duration));

}