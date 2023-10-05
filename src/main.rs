#![allow(unused_variables)]
use startup::Note;
use processing::filter_processor::{Filter,FilterType};
mod startup;
mod output;
mod processing {
    pub mod filter_processor;
}

fn main() {

    //Generating array of notes, names and frequencies
    let note_array = startup::generate_notes();

    //Generating oscillator types
    let (sine,square,saw) = startup::generate_oscillators();
    let mut notes: Vec<Note> = Vec::new();

    //Generating default filter
    let current_filter: Filter = Filter::new(FilterType::LP,5000);


    //playing music
    notes.push(note_array[50]);
    notes.push(note_array[54]);
    notes.push(note_array[57]);
    output::play_oscillator(&square, &notes, 1000, &current_filter);

    //changing filter type
    println!("filter changed");
    let current_filter: Filter = Filter::new(FilterType::HP,5000);

    //playing music
    notes.clear();
    notes.push(note_array[52]);
    notes.push(note_array[54]);
    notes.push(note_array[57]);
    output::play_oscillator(&square, &notes, 1000, &current_filter);

}