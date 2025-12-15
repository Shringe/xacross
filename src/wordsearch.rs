use strum::IntoEnumIterator;
use strum::EnumIter;
use std::fmt;

use crate::solution::Solution;



#[derive(Clone)]
pub struct Word {
    pub string: String,
    pub points: Vec<Point>,
}

#[derive(EnumIter)]
pub enum Direction {
    NorthWest,
    North,
    NorthEast,
    West,
    East,
    SouthWest,
    South,
    SouthEast,
}

#[derive(Debug, Clone)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl Point {
    pub fn is_valid(&self, direction: &Direction, wordsearch: &WordSearch) -> bool {
        match direction {
            Direction::NorthWest => self.x != 0 && self.y != 0,
            Direction::North => self.y != 0,
            Direction::NorthEast => self.x != wordsearch.width - 1 && self.y != 0,
            Direction::West => self.x != 0,
            Direction::East => self.x != wordsearch.width - 1,
            Direction::SouthWest => self.x != 0 && self.y != wordsearch.height - 1,
            Direction::South => self.y != wordsearch.height - 1,
            Direction::SouthEast => self.x != wordsearch.width - 1 && self.y != wordsearch.height - 1,
        }
    }

    pub fn follow(&self, direction: &Direction) -> Point {
        match direction {
            Direction::NorthWest => Point { x: self.x - 1, y: self.y - 1 },
            Direction::North => Point { x: self.x, y: self.y - 1 },
            Direction::NorthEast => Point { x: self.x + 1, y: self.y - 1 },
            Direction::West => Point { x: self.x - 1, y: self.y },
            Direction::East => Point { x: self.x + 1, y: self.y },
            Direction::SouthWest => Point { x: self.x - 1, y: self.y + 1 },
            Direction::South => Point { x: self.x, y: self.y + 1 },
            Direction::SouthEast => Point { x: self.x + 1, y: self.y + 1 },
        }
    }
}

#[derive(Debug, Clone)]
pub struct WordSearch { 
    pub grid: Vec<Vec<char>>,
    pub bank: Vec<Vec<char>>,
    pub width: usize,
    pub height: usize,
}

impl WordSearch {
    pub fn new(grid: Vec<Vec<char>>, bank: Vec<Vec<char>>) -> Self {
        let width = grid.len();
        let height = grid[0].len();

        Self {
            grid,
            bank,
            width,
            height,
        }
    }

    pub fn parse(raw_wordsearch: &String) -> WordSearch {
        let mut grid = Vec::new();
        let mut bank = Vec::new();

        let mut is_bank = false;
        for line in raw_wordsearch.to_uppercase().lines() {
            if line.trim().is_empty() {
                is_bank = true;
                continue;
            }

            if is_bank {
                bank.push(line.chars().filter(|c| !c.is_whitespace()).collect());
            } else {
                grid.push(line.chars().collect());
            }
        }

        WordSearch::new(grid, bank)
    }

    pub fn search(&self) -> Solution {
        let mut found = Vec::new();

        for x in 0..self.width {
            for y in 0..self.height {
                let start = Point { x, y };

                for direction in Direction::iter() {
                    let mut cpath= vec![self.grid[start.x][start.y]];
                    let mut current = start.clone();
                    let mut ppath = vec![start.clone()];

                    // Keep moving in the same direction until out of bounds
                    while current.is_valid(&direction, self) {
                        current = current.follow(&direction);
                        cpath.push(self.grid[current.x][current.y]);
                        ppath.push(current.clone());

                        // Check if we found a word
                        if self.bank.contains(&cpath) {
                            found.push( Word {
                                string: cpath.iter().collect::<String>(),
                                points: ppath.clone(),
                            })
                        } 
                    }
                }
            }
        }

        Solution::new(self.clone(), found)
    }
}
