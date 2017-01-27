use std::str::Chars;

struct Tokenizer<'a> {
	pos: usize,
	current_char:char,
	token:Vec<char>,
	length:usize,
	input:Chars<'a>,
	token_buffer:Vec<String>,
}

impl<'a> Tokenizer<'a> {

	/* tokenizer constructor
	 * Create object of type Tokenizer
	 * and returns it
	 */
	fn new(text:&str)-> Tokenizer {
		
		let token:Vec<char> = Vec::new();
		let token_stream:Vec<String> = Vec::new();

		// create structure object and initialize
		let self_object = Tokenizer {
            pos:0,
            current_char:' ',
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
        // token vector to be returned
	    // let mut token_buffer:Vec<String> =Vec::new();
        
        self.current_char = self.get_next_char();
		loop{	
			//get the character at self.pos
            //} else {
                //make sure everything is dumped
             //   break;
           // }
			
			//println!(" char {} ",self.current_char);
			
            match self.current_char {
                ' '|'\n'|'\t' => {
                    self.push_to_tok_buffer();
                    self.current_char = self.get_next_char();
                },
                t @ '"'=> {
                    self.token.push(t);
//                    self.push_to_tok_buffer();
                    let mut ch = self.get_next_char();
                    while ch!='"' {
                        self.token.push(ch);
                        ch = self.get_next_char();
                    }
                    self.token.push('\"');
					self.push_to_tok_buffer();

                    self.current_char = self.get_next_char();
                },
                t @ '{'| t @ '('| t @ '[' | t @ '}'| t @ ')'| t @ ']'=> {
                    self.token.push(t);
                    self.push_to_tok_buffer();
                    self.current_char = self.get_next_char()
				},
                t @ '<' | t @ '>' | t @ '=' => {
                    self.token.push(t);
                    let ch = self.get_next_char();
                    match ch {
                        '>' | '<' | '=' => {
                            self.token.push(ch);
                            self.current_char = self.get_next_char()
                        }
                        _ => {}
                    }
                    self.push_to_tok_buffer();
                }
                t @ 'a' ... 'z' | t @ 'A' ... 'Z' => {
                    self.token.push(t);
                    loop{
                        let ch = self.get_next_char();
                        match ch {
                            'a' ... 'z' | 'A' ... 'Z' | '0' ... '9' => {
                                self.token.push(ch);
                            }
                            _ => {
                                self.current_char = ch;
                                break;
                            }
                        }
                    }
                    self.push_to_tok_buffer();
                }
				e @  _ => {

                    self.token.push(e);		
                    self.push_to_tok_buffer();
                    self.current_char = self.get_next_char();
				},
			}
			if self.pos > self.length {
                break;
            }
		}
        
        //return the stream clone to struct internal object
		self.token_buffer.clone()	
	}

	/* get_next_token:
	 * returns the next char in a input stream
	 * pointed by `pos` position
	 */
	 fn get_next_char(&mut self)->char {
		self.pos+=1;
		if let Some(ch)=self.input.next() {
            ch
        } else {
            ' '
        }
	}
	

	/* function to put each token into
	 * the token stream as it read
	 */
	fn push_to_tok_buffer(&mut self) {
		let token:String = self.token.iter().cloned().collect();
		if !token.is_empty() {
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

    for i in tokens {
        println!["<< {} >>",i];
	}
}
