use std::fs;
use std::path::PathBuf;
use std::io::Result;


#[derive(Debug)]
pub struct WordSearch { 
    grid: Vec<char>,
    bank: Vec<String>,
    rows: usize,
    cols: usize,
}

pub fn read_wordsearch(file: &PathBuf) -> Result<String> {
    Ok(fs::read_to_string(file)?)
}

pub fn parse_wordsearch(raw_wordsearch: &String) -> WordSearch {
    let mut grid = Vec::<char>::new();
    let mut bank = Vec::<String>::new();
    let mut rows = 0;
    let mut cols = 0;

    let mut is_bank = false;
    for line in raw_wordsearch.lines() {
        if line == "" {
            is_bank = true;
            continue
        }

        if is_bank {
            bank.push(line.to_string());
        } else {
            grid.extend(line.chars().collect::<Vec<char>>());

            rows += 1;
            if cols == 0 {
                cols = line.len()
            }
        }
    }

    WordSearch {
        grid,
        bank,
        rows,
        cols,
    }
}

pub fn generate_directions(wordsearch: &WordSearch, index: usize, row: usize, col: usize) -> Vec<usize> {
    let width = wordsearch.cols;
    let height = wordsearch.rows;

    let is_first_row = row == 0;
    let is_last_row = row == height - 1;
    let is_first_col = col == 0;
    let is_last_col = col == width - 1;

    let mut points = Vec::<usize>::new();
    if !is_first_row && !is_first_col { // NW
        points.push(index - width - 1);
    } if !is_first_row { // N
        points.push(index - width);
    } if !is_first_row && !is_last_col { // NE
        points.push(index - width + 1);
    } if !is_first_col { // W
        points.push(index - 1);
    } if !is_last_col { // E
        points.push(index + 1)
    } if !is_last_row && !is_first_col { // SW
        points.push(index + width - 1)
    } if !is_last_row { // S
        points.push(index + width)
    } if !is_last_row && !is_last_col { // SE
        points.push(index + width + 1)
    }

    points
}

pub fn search(wordsearch: &WordSearch) {
    for (i, letter) in wordsearch.grid.iter().enumerate() {
        let row = i / wordsearch.cols;
        let col = i % wordsearch.cols;

        let directions = generate_directions(wordsearch,i, row, col);
        println!("{:?}", directions);
        for direction in directions {
        }
    }
}
