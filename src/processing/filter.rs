use crate::startup::WavetableOscillator;
use rodio::source::Source;
use rodio::source::BltFilter;

pub fn filter(oscillator: WavetableOscillator, frequency: u32) -> BltFilter<WavetableOscillator> {
    let filtered_oscillator = oscillator.low_pass(frequency);
    filtered_oscillator
}