// Ranks some search results by a metric.

use std::cmp::Ordering;

pub fn rank_by(words: &Vec<String>, f: &dyn Fn(&String, &String) -> Ordering) -> Vec<String> {
    let mut words = words.clone();
    words.sort_by(f);
    words
}
pub fn rank_by_length(words: &Vec<String>) -> Vec<String> {
    rank_by(words, &|w1: &String, w2: &String| w1.len().cmp(&w2.len()))
}

pub fn rank_by_score(words: &Vec<String>) -> Vec<String> {
    let mut words = words.clone();
    words.sort_by(|w1, w2| w1.len().cmp(&w2.len()));
    words
}
