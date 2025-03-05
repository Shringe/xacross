mod cli;
mod wordsearch;
mod solution;

use clap::Parser;
use wordsearch::WordSearch;
use std::fs::read_to_string;


fn main() {
    let args = cli::Cli::parse();

    if args.debug {
        println!("{:#?}", args);
    }

    let raw_wordsearch = read_to_string(&args.file).expect("Could not read file");
    let wordsearch = WordSearch::parse(&raw_wordsearch);
    let solved = wordsearch.search();

    println!("{}", solved.render(
            !args.no_grid,
            !args.no_bank
            ));

    //if !args.no_raw_grid {
    //    println!("{}", wordsearch.render());
    //} if !args.no_raw_bank {
    //
    //} if !args.no_grid {
    //    println!("{}", solved.render_grid());
    //} if !args.no_bank {
    //    println!("{}", solved.render_bank());
    //}
}
