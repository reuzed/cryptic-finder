use std::{cmp::{max_by, min_by}, collections::HashMap, fmt};

pub fn normalise(s: &str) -> String {
    // Decapitalise and remove spaces 
    s.chars().filter(|&c| c != ' ').map(|c| c.to_ascii_lowercase()).collect()
}

pub fn reverse(s: &str) -> String {
    s.chars().rev().collect()
}

pub fn lex_order(s1: &str, s2: &str) -> bool {
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


pub struct WordDecomposition {
    parts: Vec<String>
}
impl fmt::Debug for WordDecomposition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result{
        let out = self.parts.join("|");
        write!(f, "\n {}", out)
    }
}

pub fn word_decompose(s: &str, is_word: &dyn Fn(&str) -> bool) -> Vec<WordDecomposition> {
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

pub fn scrabble_score(w: &str) -> u32 {
    let scrabble_values = HashMap::from([
        ('A', 1),
        ('B', 3),
        ('C', 3),
        ('D', 2),
        ('E', 1),
        ('F', 4),
        ('G', 2),
        ('H', 4),
        ('I', 1),
        ('J', 8),
        ('K', 5),
        ('L', 1),
        ('M', 3),
        ('N', 1),
        ('O', 1),
        ('P', 3),
        ('Q', 10),
        ('R', 1),
        ('S', 1),
        ('T', 1),
        ('U', 1),
        ('V', 4),
        ('W', 4),
        ('X', 8),
        ('Y', 4),
        ('Z', 10),
    ]);
    w.chars().map(|c| scrabble_values.get(&c.to_ascii_uppercase()).unwrap()).sum()
}


