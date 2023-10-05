#![ allow(unused_variables)]
use processing::filter::FilterType;
use processing::filter::Filter;
use startup::Note;
mod startup;
mod output;
mod processing {
    pub mod filter;
}

fn main() {

    //Generating array of notes, names and frequencies
    let note_array = startup::generate_notes();

    //Generating oscillator types
    let (sine,square,saw) = startup::generate_oscillators();
    let mut notes: Vec<Note> = Vec::new();

    //changing filter type
    Filter::filter_settings(FilterType::LP);

    //playing music
    notes.push(note_array[50]);
    notes.push(note_array[54]);
    notes.push(note_array[57]);
    output::play_oscillator(&square, &notes, 100, 20000);

    //changing filter type
    Filter::filter_settings(FilterType::HP);

    //playing music
    notes.clear();
    notes.push(note_array[52]);
    notes.push(note_array[54]);
    notes.push(note_array[57]);
    output::play_oscillator(&square, &notes, 1000, 20000);
}