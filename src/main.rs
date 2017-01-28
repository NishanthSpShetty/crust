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
                '"'=> {
                    //
                    self.push_advance();
                    while self.current_char != '"' {
                        self.push_advance();
                    }
                    self.push_advance();
					self.push_to_tok_buffer();
                },
                '{'| '('| '[' | '}'| ')'| ']'=> {
                    self.push_advance();
                    self.push_to_tok_buffer();
				},
                '<' | '>' | '=' => {
                    self.push_advance();
                    match self.current_char {
                        '>' | '<' | '=' => {
                            self.push_advance();
                        }
                        _ => {}
                    }
                    self.push_to_tok_buffer();
                }
                'a' ... 'z' | 'A' ... 'Z' => {
                    self.push_advance();
                    loop {
                        match self.current_char {
                            'a' ... 'z' | 'A' ... 'Z' | '0' ... '9' => {
                                self.push_advance();
                            }
                            _ => {
                                break;
                            }
                        }
                    }
                    self.push_to_tok_buffer();
                }
                '0' ... '9' => {
                    self.push_advance();
                    if self.current_char == '.' {
                        self.push_advance();
                    }
                    loop {
                        match self.current_char {
                            '0' ... '9' => {
                                self.push_advance();
                            }
                            _ => {
                                break;
                            }
                        }
                    }
                    self.push_to_tok_buffer();
                }
                _ => {
                    self.push_advance();
                    self.push_to_tok_buffer();
                }
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

    /* push_advance:
     * push the char token passed to it onto self.token
     * gets next char and stores it in self.current_char
     */
    fn push_advance(&mut self) {
        self.token.push(self.current_char);
        self.current_char = self.get_next_char();
    }
}




fn main() {
    // let mut tok = Tokenizer{pos:-1,current_char:' ',token:&Vec<char>::new()};

    let text = "int main() {
        cout << \"Hello World\";
        int a = 125;".to_string();
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
