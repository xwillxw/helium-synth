mod startup;
mod output;
fn main() {

    //Generating array of notes, names and frequencies
    let note_array = startup::generate_notes();

    //Generating oscillator types
    let (sine,square,saw) = startup::generate_oscillators();
    
    output::play_oscillator(&sine, note_array[50]);
}