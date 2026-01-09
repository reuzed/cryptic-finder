// Find wordplay instances from a given dataset
// Have a set of target words, use some large source of words to find wordplay.
// Return structs with cerificates of the wordplay

use std::collections::{HashMap, HashSet};

#[derive(Debug)]
pub struct Anagram {
    pub fodder: String,
    pub targets: Vec<String>,
    pub anagrams: Vec<String>,
}
impl Anagram {
    pub fn canonical_target(&self) -> String {
        let mut targets = self.targets.clone();
        targets.sort();
        targets[0].clone()
    }
    pub fn unique_anagrams(&self) -> Vec<String> {
        let mut unique_anagrams = Vec::new();
        unique_anagrams.append(&mut self.targets.clone());
        unique_anagrams.append(&mut self.anagrams.clone());
        let unique_anagrams: HashSet<_> = unique_anagrams.into_iter().collect();
        unique_anagrams.into_iter().collect()
    }
}

fn sort_word(word: &str) -> String {
    let mut chars: Vec<char> = word.to_ascii_lowercase().chars().collect();
    chars.sort();
    chars.iter().collect()
}

fn sorted_anagram_map(words: &Vec<String>) -> HashMap<String, Vec<String>> {
    let mut map: HashMap<String, Vec<String>> = HashMap::new();
    for word in words.iter() {
        let sorted_word = sort_word(word);
        let fodder_anagrams = map.get_mut(&sorted_word);
        match fodder_anagrams {
            Some(fodder_anagrams_row) => fodder_anagrams_row.push(word.to_string()),
            None => {
                map.insert(sorted_word, vec![word.to_string()]);
            }
        };
    }
    map
}

pub fn find_anagrams(targets: &Vec<String>, words: &Vec<String>) -> Vec<Anagram> {
    // Use sorted(w) == sorted(v) \implies anagrams(v,w)
    // Form maps from fodder to anagrams on targets and words
    // For each target, attach the corresponding anagrams in a struct and return
    let targets_map = sorted_anagram_map(&targets);
    let words_map = sorted_anagram_map(&words);
    let mut anagrams = Vec::new();
    for (fodder, targets) in targets_map.iter() {
        match words_map.get(fodder) {
            None => (),
            Some(anas) => anagrams.push(Anagram {
                fodder: fodder.to_string(),
                targets: targets.clone(),
                anagrams: anas.to_vec(),
            }),
        }
    }
    // Ensure the word is more than just self-anagram
    anagrams
        .into_iter()
        .filter(|a| a.anagrams.len() > 1)
        .collect()
}

pub struct Concatenation {
    target: String,
    decompositions: Vec<Vec<String>>,
}

pub fn find_concatenations(targets: &Vec<String>, words: &Vec<String>) -> Vec<Concatenation>{
    // Find all instances of wordplay of the form w = v | u
    vec![]
} 

pub fn find_multiple_concatenations(targets: &Vec<String>, words: &Vec<String>) -> Vec<Concatenation>{
    // Find all instances of wordplay of the form w = v_1 | v_2 | ... v_k
    vec![]
}

pub struct Insertion {
    target: String

}

pub fn find_insertions(targets: &Vec<String>, words: &Vec<String>) -> Vec<Insertion> {
    // 
    // What data do we need for an insertion?
    // Do O(n^2) sliding window to find substrings that are words, remove and check remainder for word.
    vec![]
}

pub fn find_multiple_insertions(targets: &Vec<String>, words: &Vec<String>) -> Vec<Insertion> {
    // 
    // What data do we need for an insertion?
    // Do O(n^2) sliding window to find substrings that are words, remove and check remainder for word.
    vec![]
}

pub fn find_double_meanings(words: &Vec<String>){
    // Let ~ denote the synonym relation
    // Search for instances of failure of transitivity of ~
    // I.e. we want u ~ v ~ w but u \not ~ w
    // Among these, select examples with the smallest semantic similarity (embedding CS)

}