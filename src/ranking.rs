// Ranks some search results by a metric.

pub fn rank_by_length(words: &Vec<String>) -> Vec<String> {
    let mut words = words.clone();
    words.sort_by(|w1, w2| w1.len().cmp(&w2.len()));
    words
}