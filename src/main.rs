mod library;

use library::lexer;

fn main() {

	let text = "int main() {
        	cout << \"Hello World\";
        	int a = 125.78;
		float _floating_name=8797.78778;
		a+=100; 
		}".to_string();
    	
	let mut tok = lexer::Tokenizer::new(&text);
    	println!("Tokenizing: {}",text);
    	/*  for i in 0..text.len(){
        	println!(" {} ",text.chars().nth(i).unwrap());
		}
	*/
    
    	let tokens = tok.tokenize();

    	for i in tokens {
        	println!["[ {} ]",i];
	}
}
