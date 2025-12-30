// Find wordplay instances from a given dataset
// Have a set off target words, use some large source of words to find wordplay.
// Return structs with cerificates of the wordplay

pub fn anagrams(targets: &Vec<String>, words: &Vec<String>) -> Vec<String> {
    targets.iter().map(|s| s.to_string()).collect()
}
