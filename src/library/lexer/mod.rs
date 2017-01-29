
use std::str::Chars;

pub struct Tokenizer<'a> {
    pos: usize,
    current_char: char,
    token: Vec<char>,
    length: usize,
    input: Chars<'a>,
    token_buffer: Vec<String>,
}

impl<'a> Tokenizer<'a> {
    // tokenizer constructor
    // Create object of type Tokenizer
    // and returns it
    //
    pub fn new(text: &str) -> Tokenizer {

        let token: Vec<char> = Vec::new();
        let token_stream: Vec<String> = Vec::new();

        // create structure object and initialize
        let self_object = Tokenizer {
            pos: 0,
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
    //
    pub fn tokenize(&mut self) -> Vec<String> {

        self.current_char = self.get_next_char();
        loop {
            match self.current_char {
                ' ' | '\n' | '\t' => {
                    self.push_to_tok_buffer();
                    self.current_char = self.get_next_char();
                }
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
                    self.push_to_tok_buffer();
                }

                '\'' => {
                    self.push_advance();

                    while self.current_char != '\'' {
                        if self.current_char == '\\' {
                            self.push_advance();
                        }
                        self.push_advance();
                    }

                    self.push_advance();
                    self.push_to_tok_buffer();
                }

                '{' | '(' | '[' | '}' | ')' | ']' => {
                    self.push_advance();
                    self.push_to_tok_buffer();
                }

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
                    self.push_to_tok_buffer();
                }

                '0'...'9' => {
                    self.push_advance();
                    loop {
                        match self.current_char {
                            '0'...'9' => {
                                self.push_advance();
                            }
                            '.' => {
                                self.push_advance();
                            }
                            _ => {
                                break;
                            }
                        };
                    }
                    self.push_to_tok_buffer();
                }

                '+' | '-' | '*' | '%' | '&' | '|' | '!' => {
                    self.push_advance();
                    loop {
                        match self.current_char {
                            '+' | '=' | '-' | '&' | '|' => {
                                self.push_advance();
                            }
                            _ => {
                                break;
                            }
                        };
                    }
                    self.push_to_tok_buffer();
                }

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

                                        break;
                                    }
                                }
                            }
                        }
                        '/' => {
                            // single line comment
                            loop {
                                match self.current_char {
                                    '\n' | '\r' => {
                                        break;
                                    }
                                    _ => self.push_advance(),
                                }
                            }
                        }
                        '=' => {
                            self.push_advance();
                        }
                        _ => {}

                    };

                    self.push_to_tok_buffer();
                }
                _ => {
                    self.push_advance();
                    self.push_to_tok_buffer();
                }
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
    fn push_to_tok_buffer(&mut self) {
        let token: String = self.token.iter().cloned().collect();
        if !token.is_empty() {
            self.token_buffer.push(token);
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
}


#[cfg(test)]
mod test {
    use std::fs::File;
    use std::io::Read;
    // use std::io::Write;
    use std::io::BufReader;
    use library::lexer;

    fn read_file(path: &str) -> String {
        let file = match File::open(path) {
            Ok(f) => f,
            Err(..) => panic!("Unable to open input source file."),
        };
        let mut reader = BufReader::new(&file);
        let mut text: String = String::new();
        reader.read_to_string(&mut text);
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
        tok.push_to_tok_buffer();
        assert_eq!(0, tok.token.len());
        assert_eq!(vec!["a", ], tok.token_buffer);

        tok.push_advance();
        tok.push_to_tok_buffer();

        tok.push_advance();
        tok.push_advance();
        tok.push_advance();
        tok.push_to_tok_buffer();
        assert_eq!(0, tok.token.len());
        assert_eq!(vec!["a", "=", "\"H\""], tok.token_buffer);
    }

    #[test]
    fn test_tokenize() {
        let text = read_file("test_cases/unit_tests/test_tokenize.cpp");
        let mut tok = lexer::Tokenizer::new(&text);
        let tok_vector = vec!["class", "SomeClassName", "{", "public", ":", "SomeClassName",
                              "(", ")", "{", "}","static", "int", "a", ";", "}", ";", "int",
                              "main", "(", ")", "{",
                              "/*printf(\"hello world\");\nthis is C ..\nso */",
                              "//let write some c++", "cout", "<<",
                              "\"hello \\\\ \\t \\r \\f \\b \\\" world\\n\"", "<<", "endl",
                              ";", "int", "a", "=", "100.123", "+", "100", ";", "if", "(",
                              "a", "==", "100", "&&", "b", "==", "10", ")", "cout", "<<",
                              "\"i dont know\"", ";", "a", "=", "\'c\'", ";", "a", "=",
                              "\'\\n\'", ";", "a", "=", "\'\\\'\'", ";", "}"];
        assert_eq!(tok_vector, tok.tokenize());
    }
}