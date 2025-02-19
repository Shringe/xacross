use owo_colors::OwoColorize;
use strum::IntoEnumIterator;
use strum::EnumIter;
use itertools::Itertools;
use std::fmt;


pub struct Solution {
    wordsearch: WordSearch,

    found: Vec<Word>,
}

impl Solution {
    pub fn new(wordsearch: WordSearch, found: Vec<Word>) -> Self {
        // let mut missing = total.clone();
        // missing.retain(|w| !found.iter().any(|f| f.string == w.string));

        Self {
            wordsearch,
            found,
        }
    }

    pub fn render(&self, color: bool) -> String {
        let mut grid_render: Vec<Vec<String>> = Vec::new();

        let horizontal_gap = self.wordsearch.width.to_string().len();
        //grid_render.push(vec!["_ "..self.wordsearch.width)]);
        //
        //grid_render.push(self.wordsearch.grid.map());
        let mut top_bar = Vec::new(); 
        let mut top_spacer = Vec::new();
        for x in 0..self.wordsearch.width {
            top_bar.push(x.to_string());
            top_spacer.push("_".to_string().repeat(horizontal_gap + 2));
        }
        //grid_render.push(top_bar);
        //grid_render.push(top_spacer);
        let top_spacer = "_".to_string().repeat(self.wordsearch.width * (horizontal_gap + 1));

        for (index, row) in self.wordsearch.grid.iter().enumerate() {
            let mut new_row: Vec<String> = Vec::new();
            
            for &c in row {
                new_row.push(c.to_string());
            }
            
            grid_render.push(new_row);
        }

        for word in &self.found {
            for point in &word.points {
                let letter = grid_render[point.x][point.y].bright_blue().to_string().to_lowercase();
                if color {
                    grid_render[point.x][point.y] = letter;
                }
            }
        }

        let prefix = "n |".to_string();
        let mut rendered_grid = Vec::new();
        for (index, row) in grid_render.iter().enumerate() {
            //let prefix = format!("{:0>{}} |", index, horizontal_gap);
let prefix = format!("{:0>width$} |", format!("{:0>width$}", index, width = horizontal_gap), width = horizontal_gap);
            //let prefix = format!("{:0>{}} |", format!("{:0>{}}", index, horizontal_gap), horizontal_gap);
            let row = row.iter().join(&" ".repeat(horizontal_gap));
            rendered_grid.push(prefix + &row);
        }
        let rendered_grid = rendered_grid.join("\n");
        //let rendered_grid = grid_render.iter()
        //    .map(|row| prefix.clone() + &row.iter().join(&" ".repeat(horizontal_gap)))
        //    .collect::<Vec<String>>()
        //    .join("\n");

        let rendered_bank = format!("{}",
            self.found.iter().map(|w| format!("{}: {}", w.string.green(), w.points.iter().join(", ").yellow())).join("\n")
            // self.found.iter().map(|w| format!("{}: {} -> {}", w.string.green(), w.points[0], w.points[0]))
        );
        
        // let rendered_bank = format!("{}\n{}",
        //     self.found.iter().map(|w| w.string.green()).join("\n"),
        //     self.missing.iter().map(|w| w.string.red()).join("\n")
        // );

        format!("
Wordsearch: ==================================================
{top_spacer}
{rendered_grid}

Bank, top left = (0, 0): =====================================
{rendered_bank}
                ")
    }
}

#[derive(Clone)]
pub struct Word {
    string: String,
    points: Vec<Point>,
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
    x: usize,
    y: usize,
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
    grid: Vec<Vec<char>>,
    bank: Vec<Vec<char>>,
    width: usize,
    height: usize,
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

    pub fn render(&self) -> String {
        let rendered_grid = self.grid.iter()
            .map(|row| row.iter().join(" "))
            .collect::<Vec<String>>()
            .join("\n");
        
        let rendered_bank = self.bank.iter()
            .map(|row| row.iter().collect::<String>())
            .collect::<Vec<String>>()
            .join("\n");

        format!("
Wordsearch: ==================================================
{rendered_grid}

Bank: ========================================================
{rendered_bank}
                ")
    }
}
