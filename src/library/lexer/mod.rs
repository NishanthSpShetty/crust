
use std::str::Chars;

pub struct Tokenizer<'a> {
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
	pub fn new(text:&str)-> Tokenizer {
		
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
	pub fn tokenize(&mut self)-> Vec<String>{
            
	    self.current_char = self.get_next_char();
		loop{	
            		match self.current_char {
                	' '|'\n'|'\t' => {
                    	    self.push_to_tok_buffer();
                            self.current_char = self.get_next_char();
                        },
                	'"'=> {
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
                            },
                           _ => {},
                         }
                    	 self.push_to_tok_buffer();
		     },
                
		    '_' | 'a' ... 'z' | 'A' ... 'Z' => {
                    	self.push_advance();
                        loop {
                            match self.current_char {
                                '_' | 'a' ... 'z' | 'A' ... 'Z' | '0' ... '9' => {
                                    self.push_advance();
                            },
                            _ => {
                                break;
                            },
                          }
                       }
                      self.push_to_tok_buffer();
                    },
                    
		    '0' ... '9' => {
                    	self.push_advance();
                    	loop {
                            match self.current_char {
                            	'0' ... '9' => {
                                    self.push_advance();
                                },
                            	'.' =>{
			    	    self.push_advance();
			        },
			        _ => {
                                   break;
                                },
                            };
                    	}
                      self.push_to_tok_buffer();
		    },
                  
		  '+' | '-' | '*' | '%' | '&' | '|' | '!' => {
		  	self.push_advance();
			loop{
				match self.current_char {
					'+' | '=' | '-' | '&' | '|'  => {
						self.push_advance();
						},
					_ =>{
						break;
					    },
					};
			}
			self.push_to_tok_buffer();
		  },

		'/' => {
			self.push_advance();
			match self.current_char {
				'*' => {
					//start of multi line comment
					loop{
						self.push_advance();
						if self.current_char=='*' {
							self.push_advance();
							if self.current_char =='/'{
								self.push_advance();

								break;
								}
							}
						}
					},
				'/' => {
					//single line comment
					loop{
						match self.current_char{
							'\n'|'\r'=>{
								break;
								},
							_ => self.push_advance(),
							}
						};
					},
				'=' => {
					self.push_advance();
					},
				_ => {},
				
			};	
			
		 	self.push_to_tok_buffer(); 
		 },
		  _ => {
                      self.push_advance();
                      self.push_to_tok_buffer();
         	   },
		};
		
		if self.pos > self.length {
                	break;
            	 }
		} //loop
		
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
	
	/* move_back:
	 * move back the pointer back and pops token content
	 */
	 fn move_back(&mut self){
	 	self.current_char = self.token.pop().unwrap();
		self.pos-=1;
	}
		
}


