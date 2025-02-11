use std::fs;
use std::path::PathBuf;
use std::io::{BufRead, BufReader, Result};


#[derive(Debug)]
pub struct WordSearch { 
    grid: Vec<Vec<char>>,
    bank: Vec<Vec<char>>,
}

struct Coordinate {
    x: u16,
    y: u16,
}

pub fn read_wordsearch(file: &PathBuf) -> Result<String> {
    Ok(fs::read_to_string(file)?)
}

pub fn parse_wordsearch(raw_wordsearch: &String) -> WordSearch {
    let mut grid = Vec::<Vec<char>>::new();
    let mut bank = Vec::<Vec<char>>::new();

    let mut is_bank = false;
    for line in raw_wordsearch.lines() {
        if line == "" {
            is_bank = true;
            continue
        }

        if is_bank {
            bank.push(line.chars().collect());
        } else {
            grid.push(line.chars().collect());
        }
    }

    WordSearch {
        grid,
        bank,
    }
}

fn generate_directions(wordsearch: &WordSearch) -> Vec<Coordinate>{
    vec![
        Coordinate {
            x: 15,
            y: 20,
        }
    ]
}

pub fn search(wordsearch: &WordSearch) {
    for x in &wordsearch.grid {
        for y in &wordsearch.grid {
            for direction in generate_directions(&wordsearch) {

            }
        }
    }
}
