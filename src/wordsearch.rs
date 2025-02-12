use strum::IntoEnumIterator;
use strum::EnumIter;


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

#[derive(Debug)]
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
        for line in raw_wordsearch.to_lowercase().lines() {
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

    pub fn search(&self) {
        let mut found: usize = 0;

        for x in 0..self.width {
            for y in 0..self.height {
                let start = Point { x, y };

                for direction in Direction::iter() {
                    let mut path = vec![self.grid[start.y][start.x]];
                    let mut current = start.clone();

                    // Keep moving in the same direction until out of bounds
                    while current.is_valid(&direction, self) {
                        current = current.follow(&direction);
                        path.push(self.grid[current.y][current.x]);

                        // Check if we found a word
                        if self.bank.contains(&path) {
                            println!("Found word: {:?}", path.iter().collect::<String>());
                            found += 1;
                        } 
                    }
                }
            }
        }

        let total = self.bank.len();
        println!("Wordsearch complete!");
        println!("Total: {}", total);
        println!("Found: {}", found);
        println!("Missing: {}", total - found);
    }
}
