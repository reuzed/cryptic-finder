// use std::collections::HashSet;

use cryptic_finder::{data::{load_word_frequencies, read_word_frequencies, read_words, save_word_frequencies}, utils::{lex_order, reverse}};

// Find the common cryptic wordplays from various data sources:
// - anagram
// - reversal
// - concatenation
// - insertion

// - tube stations

fn main(){
    // let wfs = read_word_frequencies();
    // println!("wfs: {:?}", wfs)
    // let wfs = load_word_frequencies();

    save_word_frequencies()
}

