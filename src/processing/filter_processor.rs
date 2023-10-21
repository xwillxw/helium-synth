use crate::startup::WavetableOscillator;
use rodio::source::Source;
use rodio::source::BltFilter;

#[allow(dead_code)]
pub enum FilterType {
    LP,
    HP,
}

pub struct Filter {
    pub filter_type: FilterType,
    pub filter_cutoff: u32
}

impl Filter {
    pub fn new(new_filter_type: FilterType, new_filter_cutoff: u32) -> Filter {
        return Filter {
            filter_type: new_filter_type,
            filter_cutoff: new_filter_cutoff,
        };
    }
}

//  -> BltFilter<WavetableOscillator> 
pub fn apply_filter(oscillator: WavetableOscillator, current_filter: &Filter) -> BltFilter<WavetableOscillator>{
    
    let filtered_oscillator: BltFilter<WavetableOscillator>;

    match &current_filter.filter_type {

        FilterType::LP => filtered_oscillator = oscillator.low_pass(current_filter.filter_cutoff),

        FilterType::HP => filtered_oscillator = oscillator.high_pass(current_filter.filter_cutoff),

    };

    filtered_oscillator
}

