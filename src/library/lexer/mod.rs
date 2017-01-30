#[allow(dead_code)]

use library::lexeme::Token;
use library::lexeme::Type;

use std::str::Chars;

pub struct Tokenizer<'a> {
    line_no: i32,
    pos: usize,
    current_char: char,
    token: Vec<char>,
    length: usize,
    input: Chars<'a>,
    pub token_buffer: Vec<Token>,
}

impl<'a> Tokenizer<'a> {
    // tokenizer constructor
    // Create object of type Tokenizer
    // and returns it
    //
    pub fn new(text: &str) -> Tokenizer {

        let token: Vec<char> = Vec::new();
        let token_stream: Vec<Token> = Vec::new();

        // create structure object and initialize
        let self_object = Tokenizer {
            pos: 0,
            line_no: 0,
            current_char: ' ',
            length: text.len(),
            token: token,
            token_buffer: token_stream,
            input: text.chars(),
        };
        self_object
    }

    // tokenize
    // function walks over given code text and
    // returns the stream of tokens
    // trait bound Clone
    pub fn tokenize(&mut self) -> Vec<Token> {

        self.current_char = self.get_next_char();
        loop {
            match self.current_char {
                '\n' => {
                    self.line_no += 1;
                    self.current_char = self.get_next_char()
                },

                ' ' | '\t' => {
                    self.current_char = self.get_next_char();
                },

                '"' => {
                    self.push_advance();

                    while self.current_char != '"' {

                        self.push_advance();
                        if self.current_char == '\\' {
                            // 	self.token.push('$');
                            self.push_advance();
                            self.push_advance();
                        }
                    }

                    self.push_advance();
                    // 	println!(" stream : {:?}",self.token);
                    self.push_to_tok_buffer(Type::STRING);
                },

                '\'' => {
                    self.push_advance();

                    while self.current_char != '\'' {
                        if self.current_char == '\\' {
                            self.push_advance();
                        }
                        self.push_advance();
                    }

                    self.push_advance();
                    self.push_to_tok_buffer(Type::CHAR_VAL);
                },

                '{' => {
                    self.push_advance();
                    self.push_to_tok_buffer(Type::LEFT_CBRACE);
                },

                '(' => {
                    self.push_advance();
                    self.push_to_tok_buffer(Type::LEFT_BRACKET);
                },

                '[' => {
                    self.push_advance();
                    self.push_to_tok_buffer(Type::LEFT_SBRACKET);
                },

                '}' => {
                    self.push_advance();
                    self.push_to_tok_buffer(Type::RIGHT_CBRACE);
                },

                ')' => {
                    self.push_advance();
                    self.push_to_tok_buffer(Type::RIGHT_BRACKET);
                },

                ']' => {
                    self.push_advance();
                    self.push_to_tok_buffer(Type::RIGHT_SBRACKET);
                }

                '<' => {
                    self.push_advance();
                    match self.current_char {
                        '<' => {
                            self.push_advance();
                            self.push_to_tok_buffer(Type::OP_BITLSHIFT);
                        },
                        
                        '=' => {
                            self.push_advance();
                            self.push_to_tok_buffer(Type::OP_LE);
                        },

                        _ => {
                            self.push_advance();
                            self.push_to_tok_buffer(Type::OP_LT);
                        }
                    }
                },
                
                '>' => {
                    self.push_advance();
                    match self.current_char {
                        '>' => {
                            self.push_advance();
                            self.push_to_tok_buffer(Type::OP_BITRSHIFT);
                        },

                        '=' => {
                            self.push_advance();
                            self.push_to_tok_buffer(Type::OP_GE);
                        },

                        _ => {
                            self.push_advance();
                            self.push_to_tok_buffer(Type::OP_GT);
                        }
                    }
                },
                
                '=' => {
                    self.push_advance();
                    match self.current_char {
                        '=' => {
                            self.push_advance();
                            self.push_to_tok_buffer(Type::OP_EQU);
                        },

                        _ => {
                            self.push_to_tok_buffer(Type::OP_ASSIGN);
                        }
                    }
                }

                '_' | 'a'...'z' | 'A'...'Z' => {
                    self.push_advance();
                    loop {
                        match self.current_char {
                            '_' | 'a'...'z' | 'A'...'Z' | '0'...'9' => {
                                self.push_advance();
                            }
                            _ => {
                                break;
                            }
                        }
                    }
                    self.push_to_tok_buffer(Type::IDENTIFIER);
                }

                '0'...'9' => {
                    self.push_advance();
                    let mut is_int: bool = true;

                    loop {
                        match self.current_char {
                            '0'...'9' => {
                                self.push_advance();
                            }
                            '.' => {
                                self.push_advance();
                                is_int = false;
                            }
                            _ => {
                                break;
                            }
                        };
                    }
                    if is_int {
                        self.push_to_tok_buffer(Type::NUM_INT);
                    } else {
                        self.push_to_tok_buffer(Type::NUM_FLOAT);
                    }
                }

                '+' => {
                    self.push_advance();
                    match self.current_char {
                        '+' => {
                            self.push_advance();
                            self.push_to_tok_buffer(Type::OP_INC);
                        },
                        
                        '=' => {
                            self.push_advance();
                            self.push_to_tok_buffer(Type::OP_PLUSEQU);
                        },
                        _ => {
                            self.push_to_tok_buffer(Type::OP_PLUS);
                        }
                    };
                },

                '-' => {
                    self.push_advance();
                    match self.current_char {
                        '-' => {
                            self.push_advance();
                            self.push_to_tok_buffer(Type::OP_DEC);
                        },
                        
                        '=' => {
                            self.push_advance();
                            self.push_to_tok_buffer(Type::OP_MINEQU);
                        },
                        _ => {
                            self.push_to_tok_buffer(Type::OP_MINUS);
                        }
                    };
                },
                
                '*' => {
                    self.push_advance();
                    match self.current_char {
                        '=' => {
                            self.push_advance();
                            self.push_to_tok_buffer(Type::OP_MULEQU);
                        },
                        _ => {
                            self.push_to_tok_buffer(Type::OP_MUL);
                        }
                    };
                },
                
                '%' => {
                    self.push_advance();
                    match self.current_char {
                        '=' => {
                            self.push_advance();
                            self.push_to_tok_buffer(Type::OP_MODEQU);
                        },
                        _ => {
                            self.push_to_tok_buffer(Type::OP_MOD);
                        }
                    };
                },

                '~' => {
                    self.push_advance();
                    self.push_to_tok_buffer(Type::OP_BITNEG);
                },

                // could be address or bitwise operator
                '&' => {
                    self.push_advance();
                    match self.current_char {
                        '&' => {
                            self.push_advance();
                            self.push_to_tok_buffer(Type::OP_LOGAND);
                        },
                        _ => {
                            self.push_to_tok_buffer(Type::OP_BITAND);
                        }
                    };
                },
                
                '|' => {
                    self.push_advance();
                    match self.current_char {
                        '|' => {
                            self.push_advance();
                            self.push_to_tok_buffer(Type::OP_LOGOR);
                        },
                        _ => {
                            self.push_to_tok_buffer(Type::OP_BITOR);
                        }
                    };
                },
                
                '!' => {
                    self.push_advance();
                    match self.current_char {
                        '=' => {
                            self.push_advance();
                            self.push_to_tok_buffer(Type::OP_NEQ);
                        },
                        _ => {
                            self.push_to_tok_buffer(Type::OP_LOGNOT);
                        }
                    };
                },

                '/' => {
                    self.push_advance();
                    match self.current_char {
                        '*' => {
                            // start of multi line comment
                            loop {
                                self.push_advance();
                                if self.current_char == '*' {
                                    self.push_advance();
                                    if self.current_char == '/' {
                                        self.push_advance();
                                        self.push_to_tok_buffer(Type::COMMENT_MULTI);
                                        break;
                                    }
                                }
                            }
                        },

                        '/' => {
                            // single line comment
                            loop {
                                match self.current_char {
                                    '\n' | '\r' => {
                                        self.line_no += 1;
                                        self.push_to_tok_buffer(Type::COMMENT_SINGLE);
                                        break;
                                    },

                                    _ => self.push_advance(),
                                }
                            }
                        },

                        '=' => {
                            self.push_advance();
                            self.push_to_tok_buffer(Type::OP_DIVEQU);
                        },

                        _ => {
                            self.push_to_tok_buffer(Type::OP_DIV);
                        }
                    };
                },

                ';' => {
                    self.push_advance();
                    self.push_to_tok_buffer(Type::SEMICOLON);
                },

                ':' => {
                    self.push_advance();
                    self.push_to_tok_buffer(Type::COLON);
                },
                
                ',' => {
                    self.push_advance();
                    self.push_to_tok_buffer(Type::COMMA);
                },

                _ => {
                    self.push_advance();
                    self.push_to_tok_buffer(Type::OTHER);
                },
            };

            if self.pos > self.length {
                break;
            }
        } //loop

        // return the stream clone to struct internal object
        self.token_buffer.clone()
    }

    // get_next_token:
    // returns the next char in a input stream
    // pointed by `pos` position
    //
    fn get_next_char(&mut self) -> char {
        self.pos += 1;
        if let Some(ch) = self.input.next() {
            ch
        } else {
            ' '
        }
    }


    // function to put each token into
    // the token stream as it read
    //
    fn push_to_tok_buffer(&mut self, tok_type: Type) {
        let token: String = self.token.iter().cloned().collect();
        if !token.is_empty() {
            let t = Token::new(token, tok_type, self.line_no);
            self.token_buffer.push(t);
        }
        self.token.clear();
    }

    // push_advance:
    // push the char token passed to it onto self.token
    // gets next char and stores it in self.current_char
    //
    fn push_advance(&mut self) {
        self.token.push(self.current_char);
        self.current_char = self.get_next_char();
    }

    // move_back:
    // move back the pointer back and pops token content
    //
    fn move_back(&mut self) {
        self.current_char = self.token.pop().unwrap();
        self.pos -= 1;
    }

    // prev_char
    // returns the previously read character
    //
    fn prev_char(&mut self) -> char {
        *self.token.last().unwrap()
    }

    // pub fn get_token_buffer(&mut self) -> Vec<Token> {
    //     self.token_buffer.clone();
    // }
}


#[cfg(test)]
mod test {
    use std::fs::File;
    use std::io::Read;
    // use std::io::Write;
    use std::io::BufReader;
    use library::lexer;
    use library::lexeme::Type;
    use library::lexeme::Token;

    fn read_file(path: &str) -> String {
        let file = match File::open(path) {
            Ok(f) => f,
            Err(..) => panic!("Unable to open input source file."),
        };
        let mut reader = BufReader::new(&file);
        let mut text: String = String::new();
        reader.read_to_string(&mut text).expect("I dont expect anything from anyone");
        text
    }

    #[test]
    fn test_get_next_char() {
        let get_next_char = |x: &str| lexer::Tokenizer::new(&x).get_next_char();
        
        assert_eq!(' ', get_next_char(""));
        assert_eq!(' ', get_next_char(" "));
        assert_eq!('a', get_next_char("abc"));
        assert_eq!('\\', get_next_char("\\"));
        assert_eq!('\n', get_next_char("\n"));
        assert_eq!('\t', get_next_char("\t"));
        assert_eq!('\"', get_next_char("\""));
        assert_eq!('\'', get_next_char("'"));
        assert_eq!('a', get_next_char("a"));
    }

    #[test]
    fn test_struct_members() {
        let tok = lexer::Tokenizer::new("cout << \"Hello World\"");
        // pos
        assert_eq!(0, tok.pos);
        // current_char
        assert_eq!(' ', tok.current_char);
        // token
        assert_eq!(0, tok.token.len());
        // length
        assert_eq!(21, tok.length);
        // token_buffer
        assert_eq!(0, tok.token_buffer.len());
    }

    #[test]
    fn test_push_advance() {
        let mut tok = lexer::Tokenizer::new("a=\"H\"");
        // set tok.current_char from tok.get_next_char()
        // do push_advance()
        // check tok.token has first char
        // check if current_char has been advanced
        tok.current_char = tok.get_next_char();
        tok.push_advance();
        assert_eq!(tok.token, ['a']);
        assert_eq!(tok.current_char, '=');

        // check if tok.token is being populated correctly
        tok.push_advance();
        assert_eq!(tok.token, ['a', '=']);
        assert_eq!(tok.current_char, '\"');

        // go to end of input and check tok.current_char
        tok.push_advance();
        tok.push_advance();
        tok.push_advance();
        assert_eq!(tok.token, ['a', '=', '\"', 'H', '\"']);
        assert_eq!(tok.current_char, ' ');
    }

    #[test]
    fn test_push_to_tok_buffer() {
        let mut tok = lexer::Tokenizer::new("a=\"H\"");

        // set tok.current_char from tok.get_next_char()
        // do push_advance()
        // do push_to_tok_buffer()
        // check if tok.token is empty
        // check if tok.token_buffer has current token
        tok.current_char = tok.get_next_char();

        tok.push_advance();
        tok.push_to_tok_buffer(Type::IDENTIFIER);
        assert_eq!(tok.token_buffer[0].get_token_type(), Type::IDENTIFIER);
        assert_eq!(tok.token_buffer[0].get_token_value(), String::from("a"));
        assert_eq!(tok.token_buffer[0].get_token_ln(), 0);
        assert_eq!(0, tok.token.len());

        tok.push_advance();
        tok.push_to_tok_buffer(Type::OP_ASSIGN);
        assert_eq!(tok.token_buffer[1].get_token_type(), Type::OP_ASSIGN);
        assert_eq!(tok.token_buffer[1].get_token_value(), String::from("="));
        assert_eq!(tok.token_buffer[1].get_token_ln(), 0);

        tok.push_advance();
        tok.push_advance();
        tok.push_advance();
        tok.push_to_tok_buffer(Type::STRING);
        assert_eq!(tok.token_buffer[2].get_token_type(), Type::STRING);
        assert_eq!(tok.token_buffer[2].get_token_value(), String::from("\"H\""));
        assert_eq!(tok.token_buffer[2].get_token_ln(), 0);
        assert_eq!(0, tok.token.len());
    }

    #[test]
    fn test_tokenize() {
        let text = read_file("test_cases/unit_tests/test_tokenize.cpp");
        let mut tok = lexer::Tokenizer::new(&text);
        let tok_vector = vec![
                                Token::new(String::from("class"), Type::IDENTIFIER, 0),
                                Token::new(String::from("SomeClassName"), Type::IDENTIFIER, 0),
                                Token::new(String::from("{"), Type::LEFT_CBRACE, 1),
                                Token::new(String::from("public"), Type::IDENTIFIER, 2),
                                Token::new(String::from(":"), Type::COLON, 2),
                                Token::new(String::from("SomeClassName"), Type::IDENTIFIER, 3),
                                Token::new(String::from("("), Type::LEFT_BRACKET, 3),
                                Token::new(String::from(")"), Type::RIGHT_BRACKET, 3),
                                Token::new(String::from("{"), Type::LEFT_CBRACE, 4),
                                Token::new(String::from("}"), Type::RIGHT_CBRACE, 5),
                                Token::new(String::from("static"), Type::IDENTIFIER, 6),
                                Token::new(String::from("int"), Type::IDENTIFIER, 6), 
                                Token::new(String::from("a"), Type::IDENTIFIER, 6),
                                Token::new(String::from(";"), Type::SEMICOLON, 6),
								Token::new(String::from("}"), Type::RIGHT_CBRACE, 7),
								Token::new(String::from(";"), Type::SEMICOLON, 7),
								Token::new(String::from("int"), Type::IDENTIFIER, 9),
								Token::new(String::from("main"), Type::IDENTIFIER, 9),
								Token::new(String::from("("), Type::LEFT_BRACKET, 9), 
								Token::new(String::from(")"), Type::RIGHT_BRACKET, 9),
								Token::new(String::from("{"), Type::LEFT_CBRACE, 10),
								Token::new(String::from("/*printf(\"hello world\");\nthis is C ..\nso */"), Type::COMMENT_MULTI, 11),
								Token::new(String::from("//let write some c++"), Type::COMMENT_SINGLE, 13),
								Token::new(String::from("cout"), Type::IDENTIFIER, 14),
								Token::new(String::from("<<"), Type::OP_BITLSHIFT, 14),
								Token::new(String::from("\"hello \\\\ \\t \\r \\f \\b \\\" world\\n\""), Type::STRING, 14),
								Token::new(String::from("<<"), Type::OP_BITLSHIFT, 15),
								Token::new(String::from("endl"), Type::IDENTIFIER, 15),
								Token::new(String::from(";"), Type::SEMICOLON, 15),
								Token::new(String::from("int"), Type::IDENTIFIER, 16),
								Token::new(String::from("a"), Type::IDENTIFIER, 16),
								Token::new(String::from("="), Type::OP_ASSIGN, 16),
								Token::new(String::from("100.123"), Type::NUM_FLOAT, 16),
								Token::new(String::from("+"), Type::OP_PLUS, 16),
								Token::new(String::from("100"), Type::NUM_INT, 16),
								Token::new(String::from(";"), Type::SEMICOLON, 16),
								Token::new(String::from("if"), Type::IDENTIFIER, 18),
								Token::new(String::from("("), Type::LEFT_BRACKET, 18),
								Token::new(String::from("a"), Type::IDENTIFIER, 18),
								Token::new(String::from("=="), Type::OP_EQU, 18),
								Token::new(String::from("100"), Type::NUM_INT, 18),
								Token::new(String::from("&&"), Type::OP_LOGAND, 18),
								Token::new(String::from("b"), Type::IDENTIFIER, 18),
								Token::new(String::from("=="), Type::OP_EQU, 18),
								Token::new(String::from("10"), Type::NUM_INT, 18),
								Token::new(String::from(")"), Type::RIGHT_BRACKET, 18),
								Token::new(String::from("cout"), Type::IDENTIFIER, 19),
								Token::new(String::from("<<"), Type::OP_BITLSHIFT, 19),
								Token::new(String::from("\"i dont know\""), Type::STRING, 19),
								Token::new(String::from(";"), Type::SEMICOLON, 19),
								Token::new(String::from("a"), Type::IDENTIFIER, 20),
								Token::new(String::from("="), Type::OP_ASSIGN, 20),
								Token::new(String::from("\'c\'"), Type::CHAR_VAL, 20),
								Token::new(String::from(";"), Type::SEMICOLON, 20),
								Token::new(String::from("a"), Type::IDENTIFIER, 21),
								Token::new(String::from("="), Type::OP_ASSIGN, 21),
								Token::new(String::from("\'\\n\'"), Type::CHAR_VAL, 21),
								Token::new(String::from(";"), Type::SEMICOLON, 21),
								Token::new(String::from("a"), Type::IDENTIFIER, 22),
								Token::new(String::from("="), Type::OP_ASSIGN, 22) ,
								Token::new(String::from("\'\\\'\'"), Type::CHAR_VAL, 22),
								Token::new(String::from(";"), Type::SEMICOLON, 22),
								Token::new(String::from("}"), Type::RIGHT_CBRACE, 23) ];
        // for i in 0 .. tok_vector.len() {
            // assert_eq!(tok_vector[i], tok.tokenize()[i]);
        // }
        assert_eq!(tok_vector, tok.tokenize());
    }
}
