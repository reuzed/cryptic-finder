use std::{collections::HashMap, fs::{self, File}, io::Write};

use rusqlite::{Connection, Result};

fn read_file(filename: &str) -> Vec<String>{
    fs::read_to_string(filename).expect(&format!("{}", filename))
      .split("\n").map(|s| s.trim().to_string()).collect()
}

pub fn read_words() -> Vec<String>{
    read_file("words_alpha.txt")
}

struct WordRow {
    word: String,
    frequency: u64,
}

fn sqlite_select_words(table: usize) -> Result<Vec<WordRow>>{
    let conn = Connection::open("words.db")?;

    let mut stmt = conn.prepare(&format!("SELECT word_T, frequency_I FROM '{table}'"))?;

    let rows = stmt.query_map([], |row| {
        Ok((row.get::<_, String>(0)?, u64::from_str_radix(&row.get::<_, String>(1)?, 10).unwrap()))
    })?;

    let word_rows = rows.map(|row| (|(word, frequency): (String, u64)| WordRow{word, frequency})(row.unwrap()));

    Ok(word_rows.collect())
}

pub fn read_word_frequencies() -> HashMap<String, u64>{
    let mut wfs: HashMap<String, u64> = HashMap::new();
    for i in 1..37{
        for word_row in sqlite_select_words(i).unwrap().iter(){
            wfs.insert(word_row.word.clone(), word_row.frequency);
        }
    }
    wfs
}

// actually maturin works better with uv, it has some kind of integration, so:
// 1. Install uv
// 2. uv venv .venv
// 3. activate your venv (shell specific)
// 4. uv pip install maturin

// Then manually instanciating the cargo project can be avoided, just use:

// - maturin new project_name

// A cli will ask you what kind of project you want to init, choose pyo3, that's it.

// With this setup you don't even have to manually wire the pyfunction to the pymodule, you just define it inside a rust module with the pymodule proc macro.

// You also got rust doc strings embedded in your python code, so every doc string you normally make on rust pyfunction will be displayed on hover from your intellisense in the python side and also no more "unresolved library import".

// One last thing, you can also define the signature to be shown for python with some attribute macro.

fn preprocess_word_frequencies_db() -> HashMap<String, HashMap<String, u64>>{
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
    let wfs_by_rarity: HashMap<String, HashMap<String, u64>> = HashMap::from(
        wfs_buckets.map(
            |(title, lb, ub)| (title.to_string(), wfs.clone().into_iter().filter(|(_w, f)| lb <= *f && *f < ub).collect::<HashMap<String, u64>>())
        )
    );
    println!("{:?}", wfs_by_rarity.iter().map(&|(rarity, coll):(_,&HashMap<String, u64>)| format!("{} = {}", rarity, coll.len())).collect::<Vec<_>>());
    wfs_by_rarity
}

pub fn save_word_frequencies(){
    let wfs_by_rarity: HashMap<String, HashMap<String, u64>> = preprocess_word_frequencies_db();
    let wfs_json = serde_json::to_string(&wfs_by_rarity).unwrap();
    let mut file = File::create("wfs_by_rarity.json").unwrap();

    let _ = file.write_all(wfs_json.as_bytes());
}

pub fn load_word_frequencies() -> HashMap<String, HashMap<String, u64>>{
    let wfs_json = fs::read_to_string("wfs_by_rarity").unwrap();

    let wfs_by_rarity: HashMap<String, HashMap<String, u64>> = serde_json::from_str(&wfs_json).unwrap();

    wfs_by_rarity 
}
