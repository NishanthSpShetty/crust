#[allow(dead_code)]

mod library;

use std::io;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::io::BufReader;


use library::lexeme::Type;
use library::lexer;
use library::lexeme::Token;


fn main() {
    let mut input = String::new();
    println!("Enter the C/C++ file to be tokenized(for now...): ");
    io::stdin().read_line(&mut input).expect("Unable to read");

    let file = match File::open(input.trim()) {
        Ok(f) => f,
        Err(..) => panic!("Unable to open input source file."),
    };
    // get the reader
    let mut reader = BufReader::new(&file);
    let mut text: String = String::new();
    let size = reader.read_to_string(&mut text).expect("Unable to read file.");

    println!("Input file size : {}bytes ", size);

    let mut tok = lexer::Tokenizer::new(&text);
    println!("Tokenizing \
              :\n____________________________________________________________________________________\n \
              {}", text);
    println!("__________________________________________________________________________________");

    // let tokens = tok.tokenize();
    tok.tokenize();
    for i in tok.token_buffer {
        println!["[ {:?} ], ",i];
    }

    // let output: String = tok.token_buffer.join(" ");

    // let mut file = File::create("./test_cases/unit_tests/output.rs").expect("Unable to open file to write");

    // file.write_all(output.as_bytes()).expect("Unable to write to file");
}
