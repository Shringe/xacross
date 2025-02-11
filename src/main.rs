mod cli;
mod wordsearch;

use clap::Parser;
use wordsearch::{ read_wordsearch, parse_wordsearch };


fn main() {
    let args = cli::Args::parse();

    let raw_wordsearch = read_wordsearch(&args.file).expect("Could not read file");
    let wordsearch = parse_wordsearch(&raw_wordsearch);

    if args.debug {
        println!("{:#?}", args);
        println!("{:?}", wordsearch);
    }

    wordsearch::search(&wordsearch);
}
