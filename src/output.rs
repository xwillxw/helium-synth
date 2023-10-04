use rodio::{OutputStream, source::Source};
use crate::startup;

pub fn play_oscillator(oscillator_type: &startup::WavetableOscillator, note: startup::Note){
    let mut oscillator = startup::WavetableOscillator::new(oscillator_type.sample_rate.clone(), oscillator_type.wave_table.clone());
    oscillator.set_frequency(note.frequency);
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let _result = stream_handle.play_raw(oscillator.convert_samples());
    std::thread::sleep(std::time::Duration::from_millis(50));
}