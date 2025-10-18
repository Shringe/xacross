## About
Wordsearch solver. Type in a wordsearch and its word bank, get coordinate locations and visual representations of their location. An example for formatting is included.

## Motivation
I get a lot of wordsearches in school, and needed a beginner project to familiarize myself with Rust. I'm also wanting to create a chess engine in the future, and seeing how wordsearches also take place on a 2D matrix I thought this would be a good first* Rust project.

## Building
Make sure to have cargo and git installed. The executable will be found in `./target/release/xacross`. 
```sh
git clone https://github.com/Shringe/xacross.git
cd xacross
cargo bulid --release
```

## Usage
Run `./target/release/xacross --help` for help and instructions. You may want to look at the example in `./wordsearch/SpaceShuttle.txt` for help on formating your wordsearch file.

## TODO
 - Add OCR for solving wordsearches directly off of images
 - Create a prettier results srceen
