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

    if args.debug {
        //println!("{:?}", wordsearch);
        println!("{:?}", args.commands);
    }

    match &args.commands {
        cli::Commands::RenderOptions(options) => {
            println!("'myapp add' was used, name is: {:?}", options.raw);
        }
    }

    //println!("{}", wordsearch.render());
    //println!("Solving wordsearch...");
    let solved = wordsearch.search();
    //println!("{}", solved.render());
}
