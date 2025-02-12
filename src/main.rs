mod cli;
mod wordsearch;

use clap::Parser;
use wordsearch::WordSearch;
use std::fs::read_to_string;


fn main() {
    let args = cli::Args::parse();

    let raw_wordsearch = read_to_string(&args.file).expect("Could not read file");
    let wordsearch = WordSearch::parse(&raw_wordsearch);

    if args.debug {
        println!("{:#?}", args);
        println!("{:?}", wordsearch);
    }

    wordsearch.search();
}
