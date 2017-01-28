mod library;

use library::lexer;

fn main() {

	let text = "int main() {
        	cout << \"Hello World\";
        	int a = 125.78;
		float _floating_name=8797.78778;
		a+=100; 
		a/=02;
		mod = a[2]/b;
		if(a==100 &&b==100){
			/*multiline comment
treated as one single
token.*/
			//and this
			cout<<\"I dont know what  to do here\"<<endl;
		}
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
