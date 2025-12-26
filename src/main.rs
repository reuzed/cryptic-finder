use core::fmt;
use std::cmp::{max_by, min_by};
use std::fs;
use std::collections::HashSet;

fn read_file(filename: &str) -> Vec<String>{
    fs::read_to_string(filename).expect(&format!("{}", filename))
      .split("\n").map(|s| s.trim().to_string()).collect()
}

fn reverse(s: &str) -> String {
    s.chars().rev().collect()
}

fn lex_order(s1: &str, s2: &str) -> bool {
    // Check if s1 > s2 in a lexicographic order
    let shorter = min_by(s1, s2, |s1, s2| s1.len().cmp(&s2.len()));
    let longer = max_by(s1, s2, |s1, s2| s1.len().cmp(&s2.len()));

    for (i, cs) in shorter.chars().enumerate() {
        let cl = longer.chars().nth(i).unwrap();
        if cs > cl {
            return shorter == s1;
        }
        if cl > cs {
            return longer == s1;
        }
    }
    return shorter == s1;
}

struct WordDecomposition {
    parts: Vec<String>
}
impl fmt::Debug for WordDecomposition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result{
        let out = self.parts.join("|");
        write!(f, "\n {}", out)
    }
}

fn word_decompose(s: &str, is_word: &dyn Fn(&str) -> bool) -> Vec<WordDecomposition> {
    // determine whether 's' can be decomposed into multiple words
    // do dfs, checking word set membership
    let mut decompositions: Vec<WordDecomposition> = {
        if is_word(s){
            vec![WordDecomposition{parts: vec![s.to_string()]}]
        }
        else{
            vec![]
        }
    };
    for i in 1..s.len() {
        let s1 = &s[..i];
        let s2 = &s[i..];
        if !is_word(s1) {
            continue;
        }
        let sub_decomps = word_decompose(s2, is_word);
        let mut new_decomps: Vec<WordDecomposition> = Vec::new();
        for sd in sub_decomps.iter(){
            let mut parts = sd.parts.clone();
            parts.insert(0, s1.to_string());
            new_decomps.push(WordDecomposition { parts });
        }
        decompositions.append(&mut new_decomps);
    }
    return decompositions;
}

fn _palin_rev(){
    let words = read_file("words_alpha.txt");
    let words_set: HashSet<&String> = words.iter().collect();
    let mut palindromes = words.iter()
        .filter(|&s| reverse(s) == *s).collect::<Vec<&String>>();
    palindromes.sort_by(|a,b| b.len().cmp(&a.len()));

    let mut reversibles = words.iter()
        .filter(|&s| words_set.contains(&reverse(s)))
        .filter(|&s| lex_order(s, &reverse(s)))
        .filter(|&s| reverse(s) != *s)
        .collect::<Vec<&String>>();
    reversibles.sort_by(|a,b| b.len().cmp(&a.len())); 

    for word in reversibles.iter().take(100){
        println!("{} {}", word, reverse(word));
    }
}

fn main() {
    let words = read_file("words_alpha.txt");
    let words_set: HashSet<&String> = words.iter().collect();
    
    let is_word = |s: &str| words_set.contains(&s.to_string()) && (s.len() > 4);

    let mut decomp_words= words.iter()
        .take(100000)
        .filter(|w| word_decompose(*w, &is_word).len() > 1)
        .collect::<Vec<&String>>();
    
    decomp_words.sort_by(|a,b| b.len().cmp(&a.len())); 

    for word in decomp_words.iter().take(100){
        let dec = word_decompose(word, &is_word);
        println!("{}", word);
        println!("{:?}", dec)
    }
}

