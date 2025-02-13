mod cli;
mod wordsearch;
mod solution;

use clap::Parser;
use wordsearch::WordSearch;
use std::fs::read_to_string;
use owo_colors::OwoColorize;


fn main() {
    let args = cli::Args::parse();

    let raw_wordsearch = read_to_string(&args.file).expect("Could not read file");
    let wordsearch = WordSearch::parse(&raw_wordsearch);
    

    if args.debug {
        println!("{:#?}", args);
        println!("{:?}", wordsearch);
    }

    println!("{}", wordsearch.render());
    println!("Solving wordsearch...");
    let solved = wordsearch.search();
    println!("{}", solved.render(true));
}
