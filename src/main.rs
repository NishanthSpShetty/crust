use std::str::Chars;

#[allow(dead_code)]
struct Tokenizer<'a>{
	pos: usize,
	current_char:char,
	token:Vec<char>,
	length:usize,
	input:Chars<'a>,
	}

impl<'a> Tokenizer<'a>{

	fn new(text:&str)-> Tokenizer{
		
		let token:Vec<char> = Vec::new();
	
		let s = Tokenizer{pos:0,current_char:' ',length:text.len(),token:token,input:text.chars()};
		//return the self object
		s
	}

	fn tokenize(&mut self)-> Vec<String>{

		//token vector to be returned
		let mut token_buffer:Vec<String> =Vec::new();

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
					     ' '|'\n'|'\t' =>{
					     	//	self.skip_whitespace();
						 	let token:String = self.token.iter().cloned().collect();
						 	//push into token vector stream
							if !token.is_empty(){
								token_buffer.push(token);
							}
							self.token.clear();
							//push the char into current token set
						 continue;	
						 },
						 t @ '{'| t @ '('| t @ '[' | t @ '}'| t @ ')'| t @ ']'|t @ '"'  => {
						 	//convert the Vec<token> to string
						 	let token:String = self.token.iter().cloned().collect();
						 	//push into token vector stream
							
							if !token.is_empty(){
								token_buffer.push(token);
							}
							
							
							self.token.clear();
							//push the char into current token set
							self.token.push(t);

							
						 	//convert the Vec<token> to string
						 	let token:String = self.token.iter().cloned().collect();
						 	//push into token vector stream
							if !token.is_empty(){
								token_buffer.push(token);
							}
							self.token.clear();	
						},
						e @  _ => {
							self.token.push(e);		
						},
				}
			
	//		println!["Current pos : {} len {} ",self.pos,self.length];
		}
		
		
		token_buffer	
	}

	fn get_next_char(&mut self)->char {
//		let ch= self.input.nth(self.pos).unwrap();
		let ch=self.input.next().unwrap();
		self.pos+=1;
		ch
	}

	fn skip_whitespace(&mut self){
		while self.pos<self.length-1 && self.get_next_char()==' ' {
		}
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
