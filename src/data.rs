use std::{
    collections::{HashMap, HashSet},
    fs::{self, File},
    io::Write,
};

use csv::ReaderBuilder;
use rusqlite::{Connection, Result};

fn read_file(filename: &str) -> Vec<String> {
    fs::read_to_string(filename)
        .expect(&format!("{}", filename))
        .split("\n")
        .map(|s| s.trim().to_string())
        .collect()
}

pub fn read_words() -> Vec<String> {
    read_file("data/words_alpha.txt")
}

struct WordRow {
    word: String,
    frequency: u64,
}

fn sqlite_select_words(table: usize) -> Result<Vec<WordRow>> {
    let conn = Connection::open("data/words.db")?;

    let mut stmt = conn.prepare(&format!("SELECT word_T, frequency_I FROM '{table}'"))?;

    let rows = stmt.query_map([], |row| {
        Ok((
            row.get::<_, String>(0)?,
            u64::from_str_radix(&row.get::<_, String>(1)?, 10).unwrap(),
        ))
    })?;

    let word_rows = rows
        .map(|row| (|(word, frequency): (String, u64)| WordRow { word, frequency })(row.unwrap()));

    Ok(word_rows.collect())
}

pub fn read_word_frequencies() -> HashMap<String, u64> {
    let mut wfs: HashMap<String, u64> = HashMap::new();
    for i in 1..37 {
        for word_row in sqlite_select_words(i).unwrap().iter() {
            wfs.insert(word_row.word.clone(), word_row.frequency);
        }
    }
    wfs
}

pub fn read_percentile_common_words(percentile: f64) -> Vec<String> {
    // Take all words that appear with frequency at least percentile within words of their length.
    let wfs= read_word_frequencies();
    // Split by length of word
    let lengths = wfs.iter().map(|(w, _f)| w.len()).collect::<HashSet<_>>().into_iter();
    let mut percentile_common_words: Vec<String> = Vec::new();
    for length in lengths {
        let mut wfs_by_l: Vec<_> = wfs.iter().filter(|&(w, _f)| w.len() == length).collect();
        wfs_by_l.sort_by(|wf1, wf2| wf2.1.cmp(wf1.1));
        let num_words = ((wfs_by_l.len() as f64)*percentile).floor() as usize;
        for &wf in &wfs_by_l[..num_words]{
            percentile_common_words.push(wf.0.to_string())
        }
    }
    percentile_common_words
}

fn preprocess_word_frequencies_db() -> HashMap<String, HashMap<String, u64>> {
    // Separate words into frequency buckets and save to json, printing a report.
    let wfs: HashMap<String, u64> = read_word_frequencies();
    let wfs_buckets = [
        ("unique", 0, 10),
        ("rare", 10, 10_000),
        ("uncommmon", 10_000, 100_000),
        ("common", 100_000, 1_000_000),
        ("prevalent", 1_000_000, 10_000_000),
        ("ubiquitous", 10_000_000, 1_000_000_000_000),
    ];
    let wfs_by_rarity: HashMap<String, HashMap<String, u64>> =
        HashMap::from(wfs_buckets.map(|(title, lb, ub)| {
            (
                title.to_string(),
                wfs.clone()
                    .into_iter()
                    .filter(|(_w, f)| lb <= *f && *f < ub)
                    .collect::<HashMap<String, u64>>(),
            )
        }));
    println!(
        "{:?}",
        wfs_by_rarity
            .iter()
            .map(&|(rarity, coll): (_, &HashMap<String, u64>)| format!(
                "{} = {}",
                rarity,
                coll.len()
            ))
            .collect::<Vec<_>>()
    );
    wfs_by_rarity
}

pub fn save_word_frequencies() {
    let wfs_by_rarity: HashMap<String, HashMap<String, u64>> = preprocess_word_frequencies_db();
    let wfs_json = serde_json::to_string(&wfs_by_rarity).unwrap();
    let mut file = File::create("data/wfs_by_rarity.json").unwrap();

    let _ = file.write_all(wfs_json.as_bytes());
}

pub fn load_word_frequencies() -> HashMap<String, HashMap<String, u64>> {
    let wfs_json = fs::read_to_string("data/wfs_by_rarity.json").unwrap();

    let wfs_by_rarity: HashMap<String, HashMap<String, u64>> =
        serde_json::from_str(&wfs_json).unwrap();

    wfs_by_rarity
}

fn read_csv(filename: &str, col: usize) -> Vec<String> {
    let mut rdr = ReaderBuilder::new().from_path(filename).unwrap();
    let mut result = Vec::new();
    for record in rdr.records() {
        match &record {
            Ok(_) => (),
            Err(_) => continue,
        }
        let record = record.unwrap();
        let value = record.get(col);
        match value {
            Some(value) => result.push(value.to_string()),
            None => (),
        }
    }
    return result;
}

pub fn read_tube_stations() -> Vec<String> {
    read_csv("data/Stations_2022.csv", 1)
}

pub fn read_counties() -> Vec<String> {
    read_csv("data/uk-counties-list.csv", 1)
}

pub fn read_places() -> Vec<String> {
    read_csv("data/IPN_GB_2024/IPN_GB_2024.csv", 1)
}
