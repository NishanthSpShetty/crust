use std::str::Chars;

struct Tokenizer<'a>{
	pos: usize,
	current_char:char,
	token:Vec<char>,
	length:usize,
	input:Chars<'a>,
	token_buffer:Vec<String>,
	}

impl<'a> Tokenizer<'a>{

	/* tokenizer constructor
	 * Create object of type Tokenizer
	 * and returns it
	 */
	fn new(text:&str)-> Tokenizer{
		
		let token:Vec<char> = Vec::new();
		let token_stream:Vec<String> = Vec::new();
		//create structure object and initialize
		let self_object = Tokenizer{ pos:0,current_char:' ',
				   length:text.len(),
				   token:token,
				   token_buffer:token_stream,
				   input:text.chars()
				   };
		self_object
	}

	/* tokenize
	 * function walks over given code text and 
	 * returns the stream of tokens
	 */
	fn tokenize(&mut self)-> Vec<String>{

		//token vector to be returned
	//	let mut token_buffer:Vec<String> =Vec::new();

		loop{	
			//get the character at self.pos
			if self.pos<self.length-1 {
							self.current_char = self.get_next_char();
						} else {
							//make sure everything is dumped
							break;
						}
			
			//println!(" char {} ",self.current_char);
			
			match self.current_char{
					     ' '|'\n'|'\t' =>{/*
					     	//	self.skip_whitespace();
						 	let token:String = self.token.iter().cloned().collect();
						 	//push into token vector stream
							if !token.is_empty(){
								token_buffer.push(token);
							}
							self.token.clear();
							//push the char into current token set
						 */
						 self.push_to_tok_buffer();
						 	continue;	
						 },
						 t @ '{'| t @ '('| t @ '[' | t @ '}'| t @ ')'| t @ ']'|t @ '"'  => {
						 	//convert the Vec<token> to string
						 /*	let token:String = self.token.iter().cloned().collect();
						 	//push into token vector stream
							
							if !token.is_empty(){
								token_buffer.push(token);
							}
							
							
							self.token.clear();
							*/
						 self.push_to_tok_buffer();
							//push the char into current token set
							self.token.push(t);

							/*
						 	//convert the Vec<token> to string
						 	let token:String = self.token.iter().cloned().collect();
						 	//push into token vector stream
							if !token.is_empty(){
								token_buffer.push(token);
							}
							self.token.clear();	
							*/

						 self.push_to_tok_buffer();
							
						},
						e @  _ => {
							self.token.push(e);		
						},
				}
			
	//		println!["Current pos : {} len {} ",self.pos,self.length];
		}
		
		
		self.token_buffer.clone()	
	}

	
	/* get_next_token:
	 * returns the next char in a input stream
	 * pointed by `pos` position
	 */
	 fn get_next_char(&mut self)->char {
		let ch=self.input.next().unwrap();
		self.pos+=1;
		ch
	}
	

	/* function to put each token into
	 * the token stream as it read
	 */
	fn push_to_tok_buffer(&mut self){
		let token:String = self.token.iter().cloned().collect();
		if !token.is_empty(){
			self.token_buffer.push(token);
			}
		self.token.clear();
		}

}




fn main() {
   // let mut tok = Tokenizer{pos:-1,current_char:' ',token:&Vec<char>::new()};
   let text = "int main(){
    			cout<<\"hello world\";
			}".to_string();
    
   let mut tok = Tokenizer::new(&text);
    println!("Tokenizing: {}",text);
  /*  for i in 0..text.len(){
    	println!(" {} ",text.chars().nth(i).unwrap());
	}
    */
    
    let tokens = tok.tokenize();

    for i in tokens{
    		println!["<< {} >>",i];
		}
}
