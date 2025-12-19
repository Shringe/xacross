use std::collections::HashMap;

use itertools::Itertools;
use owo_colors::{OwoColorize, Rgb};

use crate::wordsearch::{Point, Word, WordSearch};

/// Blends two rgb colors together
fn blend_colors(a: Rgb, b: Rgb) -> Rgb {
    // converting to u16 to prevent overflow
    Rgb(
        ((a.0 as u16 + b.0 as u16) / 2) as u8,
        ((a.1 as u16 + b.1 as u16) / 2) as u8,
        ((a.2 as u16 + b.2 as u16) / 2) as u8,
    )
}

pub struct Solution {
    wordsearch: WordSearch,
    found: Vec<Word>,

    horizontal_digits: usize,
    horizontal_spacer: &'static str,
    vertical_digits: usize,
    final_grid_width: usize,
}

impl Solution {
    pub fn new(wordsearch: WordSearch, mut found: Vec<Word>) -> Self {
        found.sort_by(|a, b| a.string.cmp(&b.string));

        let horizontal_digits = wordsearch.width.to_string().len();
        let vertical_digits = wordsearch.height.to_string().len();
        let horizontal_spacer = " |";

        let final_grid_width = horizontal_digits
            + horizontal_spacer.len()
            + (horizontal_digits + 2) * (wordsearch.width - 1)
            + 1;

        Self {
            wordsearch,
            found,
            horizontal_digits,
            horizontal_spacer,
            vertical_digits,
            final_grid_width,
        }
    }

    /// Renders a point for display
    fn render_point(&self, point: &Point) -> String {
        format!("({:02$}, {:02$})", point.y, point.x, self.horizontal_digits)
    }

    /// Renders the word bank
    fn render_bank(&self) -> String {
        let mut longest_word_length = 0;
        for w in self.found.iter() {
            let len = w.string.len();
            if len > longest_word_length {
                longest_word_length = len;
            }
        }

        let longest_word_length = longest_word_length + 2;

        let mut entries = self.found.iter().map(|w| {
            let padding_width = longest_word_length - w.string.len();
            let padding = " ".repeat(padding_width);

            let first = w
                .points
                .first()
                .expect("Tried to render a word, but there are no points");
            let last = w.points.last().unwrap();

            format!(
                "{}:{}{} {} {}",
                w.string.color(w.color),
                padding,
                self.render_point(first),
                w.direction,
                self.render_point(last)
            )
        });

        entries.join("\n")
    }

    /// Renders the grid of characters
    fn render_grid(&self) -> String {
        let mut grid_render: Vec<Vec<String>> = Vec::new();

        // Converting the chars to strings for coloring
        for row in self.wordsearch.grid.iter() {
            let mut new_row: Vec<String> = Vec::new();

            for &c in row {
                new_row.push(c.to_string());
            }

            grid_render.push(new_row);
        }

        // Highlighting
        let mut color_map = HashMap::new();

        for word in &self.found {
            for point in &word.points {
                let (color, overlapping) = match color_map.get(&point) {
                    Some(previous_color) => {
                        let blend = blend_colors(*previous_color, word.color);
                        (blend, true)
                    }
                    None => (word.color, false),
                };

                let letter = grid_render[point.x][point.y].clone();
                let highlighted = if overlapping {
                    letter.color(color).bold().to_string()
                } else {
                    letter.color(color).to_string()
                }
                .to_lowercase();

                color_map.insert(point, color);
                grid_render[point.x][point.y] = highlighted;
            }
        }

        // Adding the gaps and final formatting
        let mut rendered_grid = Vec::with_capacity(grid_render.len());
        let column_gap = self.wordsearch.width.to_string().len() + 1;
        // let column_gap = 2;
        let column_seperator = " ".repeat(column_gap);
        for row in grid_render.iter() {
            let row = row.iter().join(&column_seperator);
            rendered_grid.push(row);
        }

        rendered_grid.join("\n")
    }

    /// Adds coordinate guides to the grid render
    fn add_guides(&self, grid: String) -> String {
        let mut rows = Vec::with_capacity(self.wordsearch.height + 1);

        let mut top_guide =
            String::with_capacity(self.final_grid_width - 1 + self.horizontal_digits);
        top_guide.push_str(
            " ".repeat(self.horizontal_digits + self.horizontal_spacer.len())
                .as_str(),
        );

        let top_guide_spacer = " ".repeat(self.horizontal_digits);
        for index in 0..self.wordsearch.width {
            top_guide.push_str(
                format!("{:02$}{}", index, top_guide_spacer, self.horizontal_digits).as_str(),
            );
        }

        rows.push(top_guide);

        for (index, row) in grid.lines().enumerate() {
            let new_row = format!(
                "{:3$}{}{}",
                index, self.horizontal_spacer, row, self.vertical_digits
            );
            rows.push(new_row);
        }

        rows.join("\n")
    }

    /// Formats a simple divider used for the final solution display
    fn format_divider(&self, title: &str) -> String {
        let spacer = ": ";
        let mut out = String::with_capacity(self.final_grid_width);
        out.push_str(title);
        out.push_str(spacer);
        out.push_str("=".repeat(self.final_grid_width - out.len()).as_str());
        out
    }

    /// Formats a final solution screen from a rendered bank and grid
    fn format_render(&self, grid: String, bank: String) -> String {
        format!(
            "{}\n{}\n\n{}\n{}",
            self.format_divider("Wordsearch"),
            grid,
            self.format_divider("Bank"),
            bank
        )
    }

    /// Renders a bank and grid, and then formats them to be displayed
    pub fn render(&self) -> String {
        self.format_render(
            Self::add_guides(&self, self.render_grid()),
            self.render_bank(),
        )
    }
}
