use itertools::Itertools;
use owo_colors::OwoColorize;

use crate::wordsearch::{Word, WordSearch};


pub struct Solution {
    wordsearch: WordSearch,
    found: Vec<Word>,
}

impl Solution {
    pub fn new(wordsearch: WordSearch, mut found: Vec<Word>) -> Self {
        found.sort_by(|a, b| a.string.cmp(&b.string));
        Self {
            wordsearch,
            found,
        }
    }

    /// Renders the word bank
    fn render_bank(&self) -> String {
        format!("{}",
            self.found.iter().map(|w| format!("{}: {}", w.string.green(), w.points.iter().join(", ").yellow())).join("\n")
        )
    }

    /// Renders the grid of characters
    fn render_grid(&self) -> String {
        let mut grid_render: Vec<Vec<String>> = Vec::new();

        let horizontal_gap = self.wordsearch.width.to_string().len();
        //grid_render.push(vec!["_ "..self.wordsearch.width)]);
        //
        //grid_render.push(self.wordsearch.grid.map());
        let mut top_bar = Vec::new(); 
        let mut top_spacer = Vec::new();
        for x in 0..self.wordsearch.width {
            top_bar.push(x.to_string());
            top_spacer.push("_".to_string());
        }
        //grid_render.push(top_bar);
        //grid_render.push(top_spacer);

        for (_index, row) in self.wordsearch.grid.iter().enumerate() {
            let mut new_row: Vec<String> = Vec::new();
            
            for &c in row {
                new_row.push(c.to_string());
            }
            
            grid_render.push(new_row);
        }

        // Coloring
        for word in &self.found {
            for point in &word.points {
                let letter = grid_render[point.x][point.y].bright_blue().to_string().to_lowercase();
                grid_render[point.x][point.y] = letter;
            }
        }

        let mut rendered_grid = Vec::new();
        let vertical_seperator = " ".repeat(horizontal_gap);
        for (index, row) in grid_render.iter().enumerate() {
            let prefix = format!("{:0>width$} |", format!("{:0>width$}", index, width = horizontal_gap), width = horizontal_gap);
            let row = row.iter().join(&vertical_seperator);
            rendered_grid.push(prefix + &row);
        }

        rendered_grid.join("\n")
    }

    /// Formats a final solution screen from a rendered bank and grid
    fn format_render(grid: String, bank: String) -> String {
        format!("Wordsearch: ==================================================
{}

Bank, top left = (0, 0): =====================================
{}", grid, bank)
    }

    /// Renders a bank and grid, and then formats them to be displayed
    pub fn render(&self) -> String {
        Self::format_render(self.render_grid(), self.render_bank())
    }
}
