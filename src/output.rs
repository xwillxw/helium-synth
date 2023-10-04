use rodio::{OutputStream, source::Source};
use crate::startup;

pub fn play_oscillator(oscillator: startup::WavetableOscillator, note: startup::Note) -> startup::WavetableOscillator{
    let mut temp_oscillator1 = startup::WavetableOscillator::new(oscillator.sample_rate.clone(), oscillator.wave_table.clone());
    temp_oscillator1.set_frequency(note.frequency);
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let _result = stream_handle.play_raw(temp_oscillator1.convert_samples());
    std::thread::sleep(std::time::Duration::from_secs(1));
    oscillator
}