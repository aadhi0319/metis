/**
 * Aadhithya Kannan
 * 28 December 2021
 * 
 * Metis, aptly named after the shapeshifting Titan, is designed to refactor code
 * such that, by definition, the generated file is syntactically different but 
 * performatically identical to the original source file. 
 * 
 * Currently, Metis supports:
 * • C
 */

use pest::Parser;
use pest::iterators::Pair;
use pest_derive::Parser;
use std::fs;

#[derive(Parser)]
#[grammar = "grammars/c.pest"]
struct CParser;

fn main() {
    let file_str: String = fs::read_to_string("test.c").unwrap();
    let file: Pair<Rule> = CParser::parse(Rule::FILE, &file_str).expect("Parsing error.").next().expect("Parsing error.");

    println!("{:#?}", file);
}