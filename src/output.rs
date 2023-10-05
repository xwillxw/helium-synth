use rodio::{OutputStream, source::Source};
use crate::startup::WavetableOscillator;
use crate::startup::Note;
use crate::processing::filter;

pub fn play_oscillator(oscillator_type: &WavetableOscillator, note: Note){

    //creating oscillator instance based on the type and note
    let mut oscillator = WavetableOscillator::new(oscillator_type.sample_rate.clone(), oscillator_type.wave_table.clone());
    oscillator.set_frequency(note.frequency);

    //applying processing

    let oscillator = filter::filter(oscillator, 20000);

    let (_stream, stream_handle) = OutputStream::try_default().unwrap();

    let _result = stream_handle.play_raw(oscillator.convert_samples());

    std::thread::sleep(std::time::Duration::from_millis(1000));

}