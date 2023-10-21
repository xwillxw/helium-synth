use core::time::Duration;
use rodio::source::Source;


pub struct WavetableOscillator {
    pub sample_rate: u32,
    pub wave_table: Vec<f32>,
    index: f32,
    index_increment: f32,
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct Note {
    name: char,
    accidental: bool,
    pub frequency: f32,
    octave: i32,
}

impl WavetableOscillator {
    pub fn new(sample_rate: u32, wave_table: Vec<f32>) -> WavetableOscillator {
        return WavetableOscillator {
            sample_rate: sample_rate,
            wave_table: wave_table,
            index: 0.0,
            index_increment: 0.0,
        };
    }

    pub fn set_frequency(&mut self, frequency: f32) {
        self.index_increment = frequency * self.wave_table.len() as f32 / self.sample_rate as f32;
    }

    fn get_sample(&mut self) -> f32 {
        let sample = self.lerp();
        self.index += self.index_increment;
        self.index %= self.wave_table.len() as f32;
        return sample;
    }

    fn lerp(&self) -> f32 {
        let truncated_index = self.index as usize;
        let next_index = (truncated_index + 1) % self.wave_table.len();
        let next_index_weight = self.index - truncated_index as f32;
        let truncated_index_weight = 1.0 - next_index_weight;
        return truncated_index_weight * self.wave_table[truncated_index] + next_index_weight * self.wave_table[next_index];
    }
} 

impl Source for WavetableOscillator {
    fn channels(&self) -> u16 {
        return 1;
    }

    fn sample_rate(&self) -> u32 {
        return self.sample_rate;
    }   

    fn current_frame_len(&self) -> Option<usize> {
        return None;
    }

    fn total_duration(&self) -> Option<Duration> {
        return None;
    }
}

impl Iterator for WavetableOscillator {
    type Item = f32;
    
    fn next(&mut self) -> Option<Self::Item> {
        return Some(self.get_sample());
    }
}

impl Note {
    fn new(name: char, accidental: bool,frequency: f32, octave: i32) -> Note {
        return Note {
            name: name,
            accidental: accidental,
            frequency: frequency,
            octave: octave,
        };
    }
}

pub fn generate_notes() -> [Note; 108] {
    //note creation
    let mut note_array: [Note; 108] = [Note::new('a', false, 27.50, 0); 108];

    //here are e templates used to generate a very large array of notes.
    let note_name_pattern: [char; 12] = ['C', 'C', 'D','D', 'E', 'F', 'F', 'G','G', 'A', 'A', 'B'];
    let accidental_pattern: [bool; 12] = [false, true, false, false, true, false, true, false, false, true, false, true];
    let note_frequency_pattern: [f32; 12] = [16.35, 17.32, 18.35, 19.45, 20.60, 21.83, 23.12, 24.50, 25.96, 27.50, 29.14, 30.87];
    let octave_multiplier_base: i32 = 2;

    //initialisation of the array used to house the frequencies of notes, and then another for the actual completed note structs

    //this changes how many notes are generated from C0

    let mut note_frequencies: [f32; 108] = [0.0; 108];

    let mut name_index = 0;
    let mut accidental_index = 0;
    let mut frequency_index = 0;
    for n in 0..note_array.len() {
        let mut oct = 0;

        //putting names, accidentals, frequencies and octaves into a list of all notes

        //this part loops the note letter array every 12 notes
        if name_index > 11 {
            name_index = name_index - 12
        }

        if accidental_index > 11 {
            accidental_index = accidental_index - 12;
        }

        //this part increases the octave by 1 every 12 notes
        if n >= 11 {
            oct = oct + (1 * (n as i32 / 12))
        }

        //this part builds the frequencies array as we go
        if frequency_index > 11 {
            frequency_index = frequency_index - 12
        }
        if n <= 11 {
            note_frequencies[n] = note_frequency_pattern[n];
        }
        else {
            note_frequencies[n] = note_frequency_pattern[frequency_index] * octave_multiplier_base.pow(oct as u32) as f32;
        }

        //println!("{},{},{}", name_index, accidental_index, n);
        //println!("{},{},{},{},", note_name_pattern[name_index], accidental_pattern[accidental_index], note_frequencies[n], oct);

        note_array[n] = Note::new(note_name_pattern[name_index], accidental_pattern[accidental_index], note_frequencies[n], oct);

        name_index = name_index + 1;
        accidental_index = accidental_index + 1;
        frequency_index = frequency_index + 1;
    }

    //for i in 0..note_array.len() {
    //    println!("{},{},{},{}", note_array[i].name, note_array[i].accidental, note_array[i].frequency, note_array[i].octave);
    //}

    note_array
}

pub fn generate_oscillators() -> (WavetableOscillator, WavetableOscillator, WavetableOscillator) {
    //stating wavetable size and declaring wavetable vectors

    let wave_table_size = 64;
    let mut sine_wave_table: Vec<f32> = Vec::with_capacity(wave_table_size);
    let mut square_wave_table: Vec<f32> = Vec::with_capacity(wave_table_size);
    let mut saw_wave_table: Vec<f32> = Vec::with_capacity(wave_table_size);

    //wavetable generatrion

    let mut wavetable_function_saw: f32 = 1.0;
    for n in 0..wave_table_size {

        //creates a sine wave of aplitude 1 and period of wave_table_size
        
        let wavetable_function_sine = (2.0 * std::f32::consts::PI * n as f32 / wave_table_size as f32).sin(); 

        //pushes the sine wave data to the sine wavetable for each iteration

        sine_wave_table.push(wavetable_function_sine);

        //pushes the sign of the current value of wavetable_function_sine as -1 or 1
        //this creates a square wave for the square wavetable

        if wavetable_function_sine >= 0.0 {
            square_wave_table.push(1.0);
        }
        else {
            square_wave_table.push(-1.0);
        }

        wavetable_function_saw = wavetable_function_saw - (1.0/wave_table_size as f32);
        saw_wave_table.push(wavetable_function_saw);
    }

    //creating oscillators with associated sample rates and wavetables
    

    let sine_oscillator = WavetableOscillator::new(44100, sine_wave_table);
    let square_oscillator = WavetableOscillator::new(44100, square_wave_table);
    let saw_oscillator = WavetableOscillator::new(44100, saw_wave_table);
    (sine_oscillator, square_oscillator, saw_oscillator)
}