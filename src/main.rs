#[allow(dead_code)]

mod library;

use std::io;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::io::BufReader;


use library::lexer;


fn main() {
    let mut input = String::new();
    print!("Enter the C/C++ file to be tokenized(for now...) : ");
    io::stdout().flush().ok().expect("");
    io::stdin().read_line(&mut input).expect("Unable to read");

    let file = match File::open(String::from("./test_cases/unit_tests/")+input.trim()) {
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

	let mut out:Vec<String> = Vec::new();
    let tokens = tok.tokenize();
    //tok.tokenize();
    let mut ln=0;
    let mut temp = String::new();
    for i in tokens {
        println!["[ {:?} ], ",i];
	//out = out + &(i.get_token_value()) as &str;
    	temp=i.get_token_value();
	if i.get_token_ln() != ln {
		
		temp="\n".to_string()+ &temp[..];
	}
	ln=i.get_token_ln();
	out.push(temp)
    }
    println!(" {:?} ",out);
    let output: String = out.join(" ");
	println!("Translated code : {}",output);
    let mut file = File::create("./test_cases/unit_tests/output.rs").expect("Unable to open file to write");

    file.write_all(output.as_bytes()).expect("Unable to write to file");
}
