#[allow(dead_code)]

use library::lexeme::Token;
use library::lexeme::Type;
use library::lexeme::Type::*;
use std::str::Chars;

pub struct Tokenizer<'a> {
    line_no: i32,
    id:i32,
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
            id:0,
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
                }

                ' ' | '\t' => {
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
                    self.push_to_tok_buffer(STRING);
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
                    self.push_to_tok_buffer(CHAR_VAL);
                }

                '{' => {
                    self.push_advance();
                    self.push_to_tok_buffer(LEFT_CBRACE);
                }

                '(' => {
                    self.push_advance();
                    self.push_to_tok_buffer(LEFT_BRACKET);
                }

                '[' => {
                    self.push_advance();
                    self.push_to_tok_buffer(LEFT_SBRACKET);
                }

                '}' => {
                    self.push_advance();
                    self.push_to_tok_buffer(RIGHT_CBRACE);
                }

                ')' => {
                    self.push_advance();
                    self.push_to_tok_buffer(RIGHT_BRACKET);
                }

                ']' => {
                    self.push_advance();
                    self.push_to_tok_buffer(RIGHT_SBRACKET);
                }

                '<' => {
                    self.push_advance();
                    match self.current_char {
                        '<' => {
                            self.push_advance();
                            self.push_to_tok_buffer(OP_BITLSHIFT);
                        }

                        '=' => {
                            self.push_advance();
                            self.push_to_tok_buffer(OP_LE);
                        }

                        _ => {
                            self.push_advance();
                            self.push_to_tok_buffer(OP_LT);
                        }
                    }
                }

                '>' => {
                    self.push_advance();
                    match self.current_char {
                        '>' => {
                            self.push_advance();
                            self.push_to_tok_buffer(OP_BITRSHIFT);
                        }

                        '=' => {
                            self.push_advance();
                            self.push_to_tok_buffer(OP_GE);
                        }

                        _ => {
                            self.push_advance();
                            self.push_to_tok_buffer(OP_GT);
                        }
                    }
                }

                '=' => {
                    self.push_advance();
                    match self.current_char {
                        '=' => {
                            self.push_advance();
                            self.push_to_tok_buffer(OP_EQU);
                        }

                        _ => {
                            self.push_to_tok_buffer(OP_ASSIGN);
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
                    let token_type = self.identify_token_type();
                    self.push_to_tok_buffer(token_type);
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
                        self.push_to_tok_buffer(NUM_INT);
                    } else {
                        self.push_to_tok_buffer(NUM_FLOAT);
                    }
                }

                '+' => {
                    self.push_advance();
                    match self.current_char {
                        '+' => {
                            self.push_advance();
                            self.push_to_tok_buffer(OP_INC);
                        }

                        '=' => {
                            self.push_advance();
                            self.push_to_tok_buffer(OP_PLUSEQU);
                        }
                        _ => {
                            self.push_to_tok_buffer(OP_PLUS);
                        }
                    };
                }

                '-' => {
                    self.push_advance();
                    match self.current_char {
                        '-' => {
                            self.push_advance();
                            self.push_to_tok_buffer(OP_DEC);
                        }

                        '=' => {
                            self.push_advance();
                            self.push_to_tok_buffer(OP_MINEQU);
                        }
                        _ => {
                            self.push_to_tok_buffer(OP_MINUS);
                        }
                    };
                }

                '*' => {
                    self.push_advance();
                    match self.current_char {
                        '=' => {
                            self.push_advance();
                            self.push_to_tok_buffer(OP_MULEQU);
                        }
                        _ => {
                            self.push_to_tok_buffer(OP_MUL);
                        }
                    };
                }

                '%' => {
                    self.push_advance();
                    match self.current_char {
                        '=' => {
                            self.push_advance();
                            self.push_to_tok_buffer(OP_MODEQU);
                        }
                        _ => {
                            self.push_to_tok_buffer(OP_MOD);
                        }
                    };
                }

                '~' => {
                    self.push_advance();
                    self.push_to_tok_buffer(OP_BITNEG);
                }

                // could be address or bitwise operator
                '&' => {
                    self.push_advance();
                    match self.current_char {
                        '&' => {
                            self.push_advance();
                            self.push_to_tok_buffer(OP_LOGAND);
                        }
                        _ => {
                            self.push_to_tok_buffer(OP_BITAND);
                        }
                    };
                }

                '|' => {
                    self.push_advance();
                    match self.current_char {
                        '|' => {
                            self.push_advance();
                            self.push_to_tok_buffer(OP_LOGOR);
                        }
                        _ => {
                            self.push_to_tok_buffer(OP_BITOR);
                        }
                    };
                }

                '!' => {
                    self.push_advance();
                    match self.current_char {
                        '=' => {
                            self.push_advance();
                            self.push_to_tok_buffer(OP_NEQ);
                        }
                        _ => {
                            self.push_to_tok_buffer(OP_LOGNOT);
                        }
                    };
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
                                        self.push_to_tok_buffer(COMMENT_MULTI);
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
                                        self.line_no += 1;
                                        self.push_to_tok_buffer(COMMENT_SINGLE);
                                        break;
                                    }

                                    _ => self.push_advance(),
                                }
                            }
                        }

                        '=' => {
                            self.push_advance();
                            self.push_to_tok_buffer(OP_DIVEQU);
                        }

                        _ => {
                            self.push_to_tok_buffer(OP_DIV);
                        }
                    };
                }

                ';' => {
                    self.push_advance();
                    self.push_to_tok_buffer(SEMICOLON);
                }

                ':' => {
                    self.push_advance();
                    self.push_to_tok_buffer(COLON);
                }

                ',' => {
                    self.push_advance();
                    self.push_to_tok_buffer(COMMA);
                }

                _ => {
                    self.push_advance();
                    self.push_to_tok_buffer(OTHER);
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


    fn identify_token_type(&self) -> Type {
        let tok: String = self.token.iter().cloned().collect();
        match tok.as_ref() {
            "int" => PRIMITIVE_INT,
            "long" => PRIMITIVE_LONG,
            "char" => PRIMITIVE_CHAR,
            "float" => PRIMITIVE_FLOAT,
            "double" => PRIMITIVE_DOUBLE,
            "short" => PRIMITIVE_SHORT,
            "bool" => PRIMITIVE_BOOL,
            "signed" => KEYWORD_SIGNED,
            "unsigned" => KEYWORD_UNSIGNED,
            "typedef" => PRIMITIVE_TYPEDEF,
            "class" => KEYWORD_CLASS,
            "break" => KEYWORD_BREAK,
            "continue" => KEYWORD_CONTINUE,
            "for" => KEYWORD_FOR,
            "while" => KEYWORD_WHILE,
            "switch" => KEYWORD_SWITCH,
            "if" => KEYWORD_IF,
            "else" => KEYWORD_ELSE,
            "do" => KEYWORD_DO,
            "public" => KEYWORD_PUBLIC,
            "private" => KEYWORD_PRIVATE,
            "protected" => KEYWORD_PROTECTED,
            "case" => KEYWORD_CASE,
            "static" => KEYWORD_STATIC,
            "const" => KEYWORD_CONST,
            "default" => KEYWORD_DEFAULT,
            "return" => KEYWORD_RETURN,
            "true" => TRUE_VAL,
            "false" => FALSE_VAL,
	    "new" => KEYWORD_NEW,
            _ => IDENTIFIER,
        }
    }

    // function to put each token into
    // the token stream as it read
    //
    fn push_to_tok_buffer(&mut self, tok_type: Type) {
        let token: String = self.token.iter().cloned().collect();
        if !token.is_empty() {
            let t = Token::new(token, tok_type, self.line_no,self.id);
            self.token_buffer.push(t);
            self.id+=1;	
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
    use library::lexeme::Type::*;
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
        tok.push_to_tok_buffer(IDENTIFIER);
        assert_eq!(tok.token_buffer[0].get_token_type(), IDENTIFIER);
        assert_eq!(tok.token_buffer[0].get_token_value(), String::from("a"));
        assert_eq!(tok.token_buffer[0].get_token_ln(), 0);
        assert_eq!(0, tok.token.len());

        tok.push_advance();
        tok.push_to_tok_buffer(OP_ASSIGN);
        assert_eq!(tok.token_buffer[1].get_token_type(), OP_ASSIGN);
        assert_eq!(tok.token_buffer[1].get_token_value(), String::from("="));
        assert_eq!(tok.token_buffer[1].get_token_ln(), 0);

        tok.push_advance();
        tok.push_advance();
        tok.push_advance();
        tok.push_to_tok_buffer(STRING);
        assert_eq!(tok.token_buffer[2].get_token_type(), STRING);
        assert_eq!(tok.token_buffer[2].get_token_value(), String::from("\"H\""));
        assert_eq!(tok.token_buffer[2].get_token_ln(), 0);
        assert_eq!(0, tok.token.len());
    }

    #[test]
    fn test_tokenize() {
        let text = read_file("test_cases/unit_tests/test_tokenize.cpp");
        let mut tok = lexer::Tokenizer::new(&text);
        let tok_vector = vec![Token::new(String::from("protected"), KEYWORD_PROTECTED, 0),
                 Token::new(String::from("class"), KEYWORD_CLASS, 0),
                 Token::new(String::from("SomeClassName"), IDENTIFIER, 0),

                 Token::new(String::from("{"), LEFT_CBRACE, 1),
                 
                 Token::new(String::from("public"), KEYWORD_PUBLIC, 2),
                 Token::new(String::from(":"), COLON, 2),
                 
                 Token::new(String::from("SomeClassName"), IDENTIFIER, 3),
                 Token::new(String::from("("), LEFT_BRACKET, 3),
                 Token::new(String::from(")"), RIGHT_BRACKET, 3),
                 
                 Token::new(String::from("{"), LEFT_CBRACE, 4),
                 
                 Token::new(String::from("}"), RIGHT_CBRACE, 5),
                 
                 Token::new(String::from("static"), KEYWORD_STATIC, 6),
                 Token::new(String::from("int"), PRIMITIVE_INT, 6),
                 Token::new(String::from("a"), IDENTIFIER, 6),
                 Token::new(String::from(";"), SEMICOLON, 6),
                 
                 Token::new(String::from("}"), RIGHT_CBRACE, 7),
                 Token::new(String::from(";"), SEMICOLON, 7),
                 
                 Token::new(String::from("int"), PRIMITIVE_INT, 9),
                 Token::new(String::from("main"), IDENTIFIER, 9),
                 Token::new(String::from("("), LEFT_BRACKET, 9),
                 Token::new(String::from(")"), RIGHT_BRACKET, 9),
                 
                 Token::new(String::from("{"), LEFT_CBRACE, 10),
                 
                 Token::new(String::from("/*printf(\"hello world\");\nthis is C ..\nso */"),
                            COMMENT_MULTI,
                            11),
                 
                 Token::new(String::from("//let write some c++"), COMMENT_SINGLE, 13),
                 
                 Token::new(String::from("cout"), IDENTIFIER, 14),
                 Token::new(String::from("<<"), OP_BITLSHIFT, 14),
                 Token::new(String::from("\"hello \\\\ \\t \\r \\f \\b \\\" world\\n\""),
                            STRING,
                            14),
                 
                 Token::new(String::from("<<"), OP_BITLSHIFT, 15),
                 Token::new(String::from("endl"), IDENTIFIER, 15),
                 Token::new(String::from(";"), SEMICOLON, 15),

                 Token::new(String::from("float"), PRIMITIVE_FLOAT, 16),
                 Token::new(String::from("a"), IDENTIFIER, 16),
                 Token::new(String::from("="), OP_ASSIGN, 16),
                 Token::new(String::from("100.123"), NUM_FLOAT, 16),
                 Token::new(String::from("+"), OP_PLUS, 16),
                 Token::new(String::from("100"), NUM_INT, 16),
                 Token::new(String::from(";"), SEMICOLON, 16),

                 Token::new(String::from("double"), PRIMITIVE_DOUBLE, 17),
                 Token::new(String::from("b"), IDENTIFIER, 17),
                 Token::new(String::from("="), OP_ASSIGN, 17),
                 Token::new(String::from("122.0253553"), NUM_FLOAT, 17),
                 Token::new(String::from("*"), OP_MUL, 17),
                 Token::new(String::from("645.7689"), NUM_FLOAT, 17),
                 Token::new(String::from("/"), OP_DIV, 17),
                 Token::new(String::from("346"), NUM_INT, 17),
                 Token::new(String::from(";"), SEMICOLON, 17),

                 Token::new(String::from("long"), PRIMITIVE_LONG, 18),
                 Token::new(String::from("c"), IDENTIFIER, 18),
                 Token::new(String::from("="), OP_ASSIGN, 18),
                 Token::new(String::from("5999999"), NUM_INT, 18),
                 Token::new(String::from(";"), SEMICOLON, 18),
                 
                 Token::new(String::from("bool"), PRIMITIVE_BOOL, 19),
                 Token::new(String::from("d"), IDENTIFIER, 19),
                 Token::new(String::from("="), OP_ASSIGN, 19),
                 Token::new(String::from("false"), FALSE_VAL, 19),
                 Token::new(String::from("||"), OP_LOGOR, 19),
                 Token::new(String::from("true"), TRUE_VAL, 19),
                 Token::new(String::from(";"), SEMICOLON, 19),

                 Token::new(String::from("unsigned"), KEYWORD_UNSIGNED, 20),
                 Token::new(String::from("short"), PRIMITIVE_SHORT, 20),
                 Token::new(String::from("short1"), IDENTIFIER, 20),
                 Token::new(String::from("="), OP_ASSIGN, 20),
                 Token::new(String::from("4"), NUM_INT, 20),
                 Token::new(String::from(";"), SEMICOLON, 20),

                 Token::new(String::from("unsigned"), KEYWORD_UNSIGNED, 21),
                 Token::new(String::from("short"), PRIMITIVE_SHORT, 21),
                 Token::new(String::from("short2"), IDENTIFIER, 21),
                 Token::new(String::from("="), OP_ASSIGN, 21),
                 Token::new(String::from("("), LEFT_BRACKET, 21),
                 Token::new(String::from("short1"), IDENTIFIER, 21),
                 Token::new(String::from("<<"), OP_BITLSHIFT, 21),
                 Token::new(String::from("1"), NUM_INT, 21),
                 Token::new(String::from(")"), RIGHT_BRACKET, 21),
                 Token::new(String::from(">>"), OP_BITRSHIFT, 21),
                 Token::new(String::from("2"), NUM_INT, 21),
                 Token::new(String::from(";"), SEMICOLON, 21),

                 Token::new(String::from("if"), KEYWORD_IF, 22),
                 Token::new(String::from("("), LEFT_BRACKET, 22),
                 Token::new(String::from("a"), IDENTIFIER, 22),
                 Token::new(String::from("=="), OP_EQU, 22),
                 Token::new(String::from("100"), NUM_INT, 22),
                 Token::new(String::from("&&"), OP_LOGAND, 22),
                 Token::new(String::from("b"), IDENTIFIER, 22),
                 Token::new(String::from("=="), OP_EQU, 22),
                 Token::new(String::from("10"), NUM_INT, 22),
                 Token::new(String::from(")"), RIGHT_BRACKET, 22),

                 Token::new(String::from("cout"), IDENTIFIER, 23),
                 Token::new(String::from("<<"), OP_BITLSHIFT, 23),
                 Token::new(String::from("\"i dont know\""), STRING, 23),
                 Token::new(String::from(";"), SEMICOLON, 23),

                 Token::new(String::from("char"), PRIMITIVE_CHAR, 25),
                 Token::new(String::from("e"), IDENTIFIER, 25),
                 Token::new(String::from("="), OP_ASSIGN, 25),
                 Token::new(String::from("\'c\'"), CHAR_VAL, 25),
                 Token::new(String::from(";"), SEMICOLON, 25),

                 Token::new(String::from("e"), IDENTIFIER, 26),
                 Token::new(String::from("="), OP_ASSIGN, 26),
                 Token::new(String::from("\'\\n\'"), CHAR_VAL, 26),
                 Token::new(String::from(";"), SEMICOLON, 26),

                 Token::new(String::from("e"), IDENTIFIER, 27),
                 Token::new(String::from("="), OP_ASSIGN, 27),
                 Token::new(String::from("\'\\\'\'"), CHAR_VAL, 27),
                 Token::new(String::from(";"), SEMICOLON, 27),

                 Token::new(String::from("switch"), KEYWORD_SWITCH, 29),
                 Token::new(String::from("("), LEFT_BRACKET, 29),
                 Token::new(String::from("a"), IDENTIFIER, 29),
                 Token::new(String::from(")"), RIGHT_BRACKET, 29),
                 Token::new(String::from("{"), LEFT_CBRACE, 29),

                 Token::new(String::from("case"), KEYWORD_CASE, 30),
                 Token::new(String::from("\'\\n\'"), CHAR_VAL, 30),
                 Token::new(String::from(":"), COLON, 30),
                 Token::new(String::from("do_something"), IDENTIFIER, 30),
                 Token::new(String::from("("), LEFT_BRACKET, 30),
                 Token::new(String::from(")"), RIGHT_BRACKET, 30),
                 Token::new(String::from(";"), SEMICOLON, 30),

                 Token::new(String::from("break"), KEYWORD_BREAK, 31),
                 Token::new(String::from(";"), SEMICOLON, 31),

                 Token::new(String::from("default"), KEYWORD_DEFAULT, 32),
                 Token::new(String::from(":"), COLON, 32),

                 Token::new(String::from("do_the_same_damn_thing"), IDENTIFIER, 33),
                 Token::new(String::from("("), LEFT_BRACKET, 33),
                 Token::new(String::from(")"), RIGHT_BRACKET, 33),
                 Token::new(String::from(";"), SEMICOLON, 33),

                 Token::new(String::from("}"), RIGHT_CBRACE, 34),

                 Token::new(String::from("while"), KEYWORD_WHILE, 36),
                 Token::new(String::from("("), LEFT_BRACKET, 36),
                 Token::new(String::from(")"), RIGHT_BRACKET, 36),
                 Token::new(String::from("{"), LEFT_CBRACE, 36),

                 Token::new(String::from("continue"), KEYWORD_CONTINUE, 37),
                 Token::new(String::from(";"), SEMICOLON, 37),

                 Token::new(String::from("}"), RIGHT_CBRACE, 38),

                 Token::new(String::from("do"), KEYWORD_DO, 39),
                 Token::new(String::from("{"), LEFT_CBRACE, 39),
                 Token::new(String::from("}"), RIGHT_CBRACE, 39),
                 Token::new(String::from("while"), KEYWORD_WHILE, 39),
                 Token::new(String::from("("), LEFT_BRACKET, 39),
                 Token::new(String::from("1"), NUM_INT, 39),
                 Token::new(String::from(")"), RIGHT_BRACKET, 39),
                 Token::new(String::from(";"), SEMICOLON, 39),
                 Token::new(String::from("}"), RIGHT_CBRACE, 41),];
        // for i in 0 .. tok_vector.len() {
        // assert_eq!(tok_vector[i], tok.tokenize()[i]);
        // }
        assert_eq!(tok_vector, tok.tokenize());
    }
}
