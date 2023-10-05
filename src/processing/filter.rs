use crate::startup::WavetableOscillator;
use rodio::source::Source;
use rodio::source::BltFilter;

pub enum FilterType {
    LP,
    HP,
}
pub struct Filter {
    filter_type: FilterType,
    filter_frequency: u32
}

impl Filter {
    pub fn filter_settings(new_filter_type: FilterType) {
        let current_filter_type = new_filter_type;
    }
    
    pub fn apply_filter(oscillator: WavetableOscillator, frequency: u32) -> BltFilter<WavetableOscillator> {
        match 
        let filtered_oscillator = oscillator.low_pass(frequency);
        filtered_oscillator
    }
}

