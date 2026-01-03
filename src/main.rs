// use std::collections::HashSet;

use cryptic_finder::{
    data::{
        load_word_frequencies, read_counties, read_places, read_tube_stations,
        read_word_frequencies, read_words, save_word_frequencies,
    },
    examples::maximally_anagrammed_by_length,
    utils::{lex_order, reverse},
};

// Find the common cryptic wordplays from various data sources:
// - anagram
// - reversal
// - concatenation
// - insertion

// - tube stations

fn main() {
    // let wfs = read_word_frequencies();
    // println!("wfs: {:?}", wfs)
    // let wfs = load_word_frequencies();

    // save_word_frequencies();

    // let tubes = read_tube_stations();
    // let places = read_places();
    // let counties = read_counties();
    // println!("{:?}", &tubes[..10]);
    // println!("{:?}", &places[..10]);
    // println!("{:?}", &counties[..10]);

    maximally_anagrammed_by_length();
}
