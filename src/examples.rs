use std::collections::HashSet;

use crate::{
    data::read_words,
    utils::{lex_order, reverse, word_decompose},
    wordplay::find_anagrams,
};

fn _palin_rev() {
    let words = read_words();
    let words_set: HashSet<&String> = words.iter().collect();
    let mut palindromes = words
        .iter()
        .filter(|&s| reverse(s) == *s)
        .collect::<Vec<&String>>();
    palindromes.sort_by(|a, b| b.len().cmp(&a.len()));

    let mut reversibles = words
        .iter()
        .filter(|&s| words_set.contains(&reverse(s)))
        .filter(|&s| lex_order(s, &reverse(s)))
        .filter(|&s| reverse(s) != *s)
        .collect::<Vec<&String>>();
    reversibles.sort_by(|a, b| b.len().cmp(&a.len()));

    for word in reversibles.iter().take(100) {
        println!("{} {}", word, reverse(word));
    }
}

fn _decomps() {
    let words = read_words();
    let words_set: HashSet<&String> = words.iter().collect();

    let is_word = |s: &str| words_set.contains(&s.to_string()) && (s.len() > 4);

    let mut decomp_words = words
        .iter()
        .take(100000)
        .filter(|w| word_decompose(*w, &is_word).len() > 1)
        .collect::<Vec<&String>>();

    decomp_words.sort_by(|a, b| b.len().cmp(&a.len()));

    for &word in decomp_words.iter().take(100) {
        let dec = word_decompose(word, &is_word);
        println!("{}", word);
        println!("{:?}", dec)
    }
}

pub fn maximally_anagrammed_by_length() {
    // Find the words which have most anagrams of each length
    let words = read_words();
    let anagrams = find_anagrams(&words, &words);
    let lengths: HashSet<usize> = anagrams
        .iter()
        .map(|a| (&a.fodder).chars().count())
        .collect();
    let mut lengths: Vec<_> = lengths.iter().collect();
    lengths.sort();
    for &&length in lengths.iter() {
        let mut anagrams: Vec<_> = anagrams
            .iter()
            .filter(|&a| (&a.fodder).chars().count() == length)
            .collect();
        anagrams.sort_by(|&a, &b| (b.anagrams.len()).cmp(&a.anagrams.len()));
        let anagrams: Vec<_> = anagrams.iter().take(1).collect();
        println!("Length {}", length);
        for anagram in anagrams.iter() {
            println!("Fodder: {}", anagram.fodder);
            for target in &anagram.unique_anagrams() {
                println!("- {}", target);
            }
        }
    }
}
