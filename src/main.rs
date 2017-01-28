mod library;

use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::io::BufReader;



use library::lexer;

fn main() {

	let file = match File::open("./test_code/test.cpp"){
		Ok(f) => f,
		Err(..) => panic!("Unable to open input source file."),
		};
	//get the reader
	let mut reader = BufReader::new(&file);
	let mut text:String = String::new();
	let size = reader.read_to_string(&mut text).expect("");
	
	println!("Input file size : {}bytes ",size);
		
	let mut tok = lexer::Tokenizer::new(&text);
    	println!("Tokenizing :\n____________________________________________________________________________________
	\n {}",text);
	println!("__________________________________________________________________________________");
    
    	let tokens = tok.tokenize();

    	for i in &tokens {
        	println!["[ {} ], ",i];
	}

	let output:String = tokens.join(" ");

	let mut file = File::create("./test_code/output.rs").expect("Unable to open file to write");

	file.write_all(output.as_bytes()).expect("Unable to write to file");
}
