use rodio::{OutputStream, source::Source, PlayError};
use crate::startup::WavetableOscillator;
use crate::startup::Note;
use crate::processing::filter;

pub fn play_oscillator(oscillator_type: &WavetableOscillator, notes: Vec<Note>){

    let (_stream, stream_handle) = OutputStream::try_default().unwrap();

    let mut output: Vec<Result<(), PlayError>> = Vec::new();

    for i in 0..notes.len() {
        let mut oscillator = WavetableOscillator::new(oscillator_type.sample_rate.clone(), oscillator_type.wave_table.clone());
        oscillator.set_frequency(notes[i].frequency);
        let oscillator = filter::filter(oscillator, 1000).convert_samples::<f32>();
        let oscillator = stream_handle.play_raw(oscillator);
        output.push(oscillator);
    }
    
    //applying processing

    let _result: &Result<(), PlayError> = &output[1];

    std::thread::sleep(std::time::Duration::from_millis(1000));

}