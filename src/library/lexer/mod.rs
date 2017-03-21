#[allow(dead_code)]

use library::lexeme::Token;
use library::lexeme::Type;
use library::lexeme::Type::*;
use std::str::Chars;

pub struct Tokenizer<'a> {
    line_no: u32,
    id: u32,
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
            id: 0,
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
                    self.push_to_tok_buffer(STRING, BASE_VALUE);
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
                    self.push_to_tok_buffer(CHAR_VAL, BASE_VALUE);
                }

                '{' => {
                    self.push_advance();
                    self.push_to_tok_buffer(LEFT_CBRACE, BASE_NONE);
                }

                '(' => {
                    self.push_advance();
                    self.push_to_tok_buffer(LEFT_BRACKET, BASE_NONE);
                }

                '[' => {
                    self.push_advance();
                    self.push_to_tok_buffer(LEFT_SBRACKET, BASE_NONE);
                }

                '}' => {
                    self.push_advance();
                    self.push_to_tok_buffer(RIGHT_CBRACE, BASE_NONE);
                }

                ')' => {
                    self.push_advance();
                    self.push_to_tok_buffer(RIGHT_BRACKET, BASE_NONE);
                }

                ']' => {
                    self.push_advance();
                    self.push_to_tok_buffer(RIGHT_SBRACKET, BASE_NONE);
                }

                '<' => {
                    self.push_advance();
                    match self.current_char {
                        '<' => {
                            self.push_advance();
                            self.push_to_tok_buffer(OP_BITLSHIFT, BASE_BINOP);
                        }

                        '=' => {
                            self.push_advance();
                            self.push_to_tok_buffer(OP_LE, BASE_BINOP);
                        }

                        _ => {
                            self.push_to_tok_buffer(OP_LT, BASE_BINOP);
                        }
                    }
                }

                '>' => {
                    self.push_advance();
                    match self.current_char {
                        '>' => {
                            self.push_advance();
                            self.push_to_tok_buffer(OP_BITRSHIFT, BASE_BINOP);
                        }

                        '=' => {
                            self.push_advance();
                            self.push_to_tok_buffer(OP_GE, BASE_BINOP);
                        }

                        _ => {
                            self.push_to_tok_buffer(OP_GT, BASE_BINOP);
                        }
                    }
                }

                '=' => {
                    self.push_advance();
                    match self.current_char {
                        '=' => {
                            self.push_advance();
                            self.push_to_tok_buffer(OP_EQU, BASE_BINOP);
                        }

                        _ => {
                            self.push_to_tok_buffer(OP_ASSIGN, BASE_NONE);
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
                    let (token_type, base_type) = self.identify_token_type();
                    self.push_to_tok_buffer(token_type, base_type);
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
                        self.push_to_tok_buffer(NUM_INT, BASE_VALUE);
                    } else {
                        self.push_to_tok_buffer(NUM_FLOAT, BASE_VALUE);
                    }
                }

                '+' => {
                    self.push_advance();
                    match self.current_char {
                        '+' => {
                            self.push_advance();
                            self.push_to_tok_buffer(OP_INC, BASE_UNOP);
                        }

                        '=' => {
                            self.push_advance();
                            self.push_to_tok_buffer(OP_PLUSEQU, BASE_ASSIGNOP);
                        }
                        _ => {
                            self.push_to_tok_buffer(OP_PLUS, BASE_BINOP);
                        }
                    };
                }

                '-' => {
                    self.push_advance();
                    match self.current_char {
                        '-' => {
                            self.push_advance();
                            self.push_to_tok_buffer(OP_DEC, BASE_UNOP);
                        }

                        '=' => {
                            self.push_advance();
                            self.push_to_tok_buffer(OP_MINEQU, BASE_ASSIGNOP);
                        }
                        _ => {
                            self.push_to_tok_buffer(OP_MINUS, BASE_BINOP);
                        }
                    };
                }

                '*' => {
                    self.push_advance();
                    match self.current_char {
                        '=' => {
                            self.push_advance();
                            self.push_to_tok_buffer(OP_MULEQU, BASE_ASSIGNOP);
                        }
                        _ => {
                            self.push_to_tok_buffer(OP_MUL, BASE_BINOP);
                        }
                    };
                }

                '%' => {
                    self.push_advance();
                    match self.current_char {
                        '=' => {
                            self.push_advance();
                            self.push_to_tok_buffer(OP_MODEQU, BASE_ASSIGNOP);
                        }
                        _ => {
                            self.push_to_tok_buffer(OP_MOD, BASE_BINOP);
                        }
                    };
                }

                '~' => {
                    self.push_advance();
                    self.push_to_tok_buffer(OP_BITNEG, BASE_UNOP);
                }

                // could be address or bitwise operator
                '&' => {
                    self.push_advance();
                    match self.current_char {
                        '&' => {
                            self.push_advance();
                            self.push_to_tok_buffer(OP_LOGAND, BASE_BINOP);
                        }
                        _ => {
                            self.push_to_tok_buffer(OP_BITAND, BASE_BINOP);
                        }
                    };
                }

                '|' => {
                    self.push_advance();
                    match self.current_char {
                        '|' => {
                            self.push_advance();
                            self.push_to_tok_buffer(OP_LOGOR, BASE_BINOP);
                        }
                        _ => {
                            self.push_to_tok_buffer(OP_BITOR, BASE_BINOP);
                        }
                    };
                }

                '!' => {
                    self.push_advance();
                    match self.current_char {
                        '=' => {
                            self.push_advance();
                            self.push_to_tok_buffer(OP_NEQ, BASE_BINOP);
                        }
                        _ => {
                            self.push_to_tok_buffer(OP_LOGNOT, BASE_UNOP);
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
                                        self.push_to_tok_buffer(COMMENT_MULTI, BASE_COMMENT);
                                        break;
                                    }
                                }
                            }
                        }

                        '/' => {
                            // single line comment
                            loop {
                                match self.current_char {
                                    '\n' | '\r' | '\0' => {
                                        self.push_to_tok_buffer(COMMENT_SINGLE, BASE_COMMENT);
                                        self.current_char = self.get_next_char();
                                        self.line_no += 1;
                                        break;
                                    }

                                    _ => self.push_advance(),
                                }
                            }
                        }

                        '=' => {
                            self.push_advance();
                            self.push_to_tok_buffer(OP_DIVEQU, BASE_ASSIGNOP);
                        }

                        _ => {
                            self.push_to_tok_buffer(OP_DIV, BASE_BINOP);
                        }
                    };
                }

                ';' => {
                    self.push_advance();
                    self.push_to_tok_buffer(SEMICOLON, BASE_NONE);
                }

                ':' => {
                    self.push_advance();
                    self.push_to_tok_buffer(COLON, BASE_NONE);
                }

                ',' => {
                    self.push_advance();
                    self.push_to_tok_buffer(COMMA, BASE_NONE);
                }
                '#' => {
                    self.push_advance();
                    self.push_to_tok_buffer(INCLUDE, BASE_NONE);
                }

                _ => {
                    self.push_advance();
                    self.push_to_tok_buffer(OTHER, BASE_NONE);
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
            '\0'
        }
    }


    fn identify_token_type(&self) -> (Type, Type) {
        let tok: String = self.token.iter().cloned().collect();
        match tok.as_ref() {
            "int" => (PRIMITIVE_INT, BASE_DATATYPE),
            "long" => (PRIMITIVE_LONG, BASE_DATATYPE),
            "char" => (PRIMITIVE_CHAR, BASE_DATATYPE),
            "float" => (PRIMITIVE_FLOAT, BASE_DATATYPE),
            "double" => (PRIMITIVE_DOUBLE, BASE_DATATYPE),
            "short" => (PRIMITIVE_SHORT, BASE_DATATYPE),
            "bool" => (PRIMITIVE_BOOL, BASE_DATATYPE),
            "signed" => (KEYWORD_SIGNED, BASE_NONE),
            "unsigned" => (KEYWORD_UNSIGNED, BASE_NONE),
            "typedef" => (PRIMITIVE_TYPEDEF, BASE_NONE),
            "class" => (KEYWORD_CLASS, BASE_NONE),
            "enum" => (KEYWORD_ENUM, BASE_NONE),
            "break" => (KEYWORD_BREAK, BASE_NONE),
            "continue" => (KEYWORD_CONTINUE, BASE_NONE),
            "for" => (KEYWORD_FOR, BASE_NONE),
            "while" => (KEYWORD_WHILE, BASE_NONE),
            "switch" => (KEYWORD_SWITCH, BASE_NONE),
            "if" => (KEYWORD_IF, BASE_NONE),
            "else" => (KEYWORD_ELSE, BASE_NONE),
            "do" => (KEYWORD_DO, BASE_NONE),
            "public" => (KEYWORD_PUBLIC, BASE_MODIFIER),
            "private" => (KEYWORD_PRIVATE, BASE_MODIFIER),
            "protected" => (KEYWORD_PROTECTED, BASE_MODIFIER),
            "case" => (KEYWORD_CASE, BASE_NONE),
            "static" => (KEYWORD_STATIC, BASE_NONE),
            "const" => (KEYWORD_CONST, BASE_NONE),
            "default" => (KEYWORD_DEFAULT, BASE_NONE),
            "return" => (KEYWORD_RETURN, BASE_NONE),
            "true" => (TRUE_VAL, BASE_VALUE),
            "false" => (FALSE_VAL, BASE_VALUE),
            "new" => (KEYWORD_NEW, BASE_NONE),
            "main" => (MAIN, BASE_NONE),
            "void" => (PRIMITIVE_VOID, BASE_DATATYPE),
            "struct" => (KEYWORD_STRUCT, BASE_NONE),
            "NULL" => (NULL, BASE_NONE),
            _ => (IDENTIFIER, BASE_NONE),
        }
    }

    // function to put each token into
    // the token stream as it read
    //
    fn push_to_tok_buffer(&mut self, tok_type: Type, base_type: Type) {
        let token: String = self.token.iter().cloned().collect();
        if !token.is_empty() {
            let t = Token::new(token, base_type, tok_type, self.line_no, self.id);
            self.token_buffer.push(t);
            self.id += 1;
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

        assert_eq!('\0', get_next_char(""));
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
        assert_eq!(tok.current_char, '\0');
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
        tok.push_to_tok_buffer(IDENTIFIER, BASE_NONE);
        assert_eq!(tok.token_buffer[0].get_token_type(), IDENTIFIER);
        assert_eq!(tok.token_buffer[0].get_base_type(), BASE_NONE);
        assert_eq!(tok.token_buffer[0].get_token_value(), String::from("a"));
        assert_eq!(tok.token_buffer[0].get_token_ln(), 0);
        assert_eq!(0, tok.token.len());

        tok.push_advance();
        tok.push_to_tok_buffer(OP_ASSIGN, BASE_NONE);
        assert_eq!(tok.token_buffer[1].get_token_type(), OP_ASSIGN);
        assert_eq!(tok.token_buffer[1].get_base_type(), BASE_NONE);
        assert_eq!(tok.token_buffer[1].get_token_value(), String::from("="));
        assert_eq!(tok.token_buffer[1].get_token_ln(), 0);

        tok.push_advance();
        tok.push_advance();
        tok.push_advance();
        tok.push_to_tok_buffer(STRING, BASE_NONE);
        assert_eq!(tok.token_buffer[2].get_token_type(), STRING);
        assert_eq!(tok.token_buffer[2].get_base_type(), BASE_NONE);
        assert_eq!(tok.token_buffer[2].get_token_value(), String::from("\"H\""));
        assert_eq!(tok.token_buffer[2].get_token_ln(), 0);
        assert_eq!(0, tok.token.len());
    }

    #[test]
    fn test_tokenize_keywords() {
        let text = read_file("test_cases/unit_tests/tokenize_keywords.cpp");
        let mut tok = lexer::Tokenizer::new(&text);
        let tok_vector =
            vec![Token::new(String::from("signed"), BASE_NONE, KEYWORD_SIGNED, 0, 0),
                 Token::new(String::from("unsigned"), BASE_NONE, KEYWORD_UNSIGNED, 1, 1),
                 Token::new(String::from("class"), BASE_NONE, KEYWORD_CLASS, 2, 2),
                 Token::new(String::from("new"), BASE_NONE, KEYWORD_NEW, 3, 3),
                 Token::new(String::from("while"), BASE_NONE, KEYWORD_WHILE, 4, 4),
                 Token::new(String::from("for"), BASE_NONE, KEYWORD_FOR, 5, 5),
                 Token::new(String::from("do"), BASE_NONE, KEYWORD_DO, 6, 6),
                 Token::new(String::from("break"), BASE_NONE, KEYWORD_BREAK, 7, 7),
                 Token::new(String::from("continue"), BASE_NONE, KEYWORD_CONTINUE, 8, 8),
                 Token::new(String::from("switch"), BASE_NONE, KEYWORD_SWITCH, 9, 9),
                 Token::new(String::from("if"), BASE_NONE, KEYWORD_IF, 10, 10),
                 Token::new(String::from("else"), BASE_NONE, KEYWORD_ELSE, 11, 11),
                 Token::new(String::from("public"),
                            BASE_MODIFIER,
                            KEYWORD_PUBLIC,
                            12,
                            12),
                 Token::new(String::from("private"),
                            BASE_MODIFIER,
                            KEYWORD_PRIVATE,
                            13,
                            13),
                 Token::new(String::from("protected"),
                            BASE_MODIFIER,
                            KEYWORD_PROTECTED,
                            14,
                            14),
                 Token::new(String::from("case"), BASE_NONE, KEYWORD_CASE, 15, 15),
                 Token::new(String::from("static"), BASE_NONE, KEYWORD_STATIC, 16, 16),
                 Token::new(String::from("const"), BASE_NONE, KEYWORD_CONST, 17, 17),
                 Token::new(String::from("default"), BASE_NONE, KEYWORD_DEFAULT, 18, 18),
                 Token::new(String::from("return"), BASE_NONE, KEYWORD_RETURN, 19, 19)];
        assert_eq!(tok_vector, tok.tokenize());
    }

    #[test]
    fn test_tokenize_types() {
        let text = read_file("test_cases/unit_tests/tokenize_types.cpp");
        let mut tok = lexer::Tokenizer::new(&text);
        let tok_vector =
            vec![Token::new(String::from("int"), BASE_DATATYPE, PRIMITIVE_INT, 0, 0),
                 Token::new(String::from("short"), BASE_DATATYPE, PRIMITIVE_SHORT, 1, 1),
                 Token::new(String::from("long"), BASE_DATATYPE, PRIMITIVE_LONG, 2, 2),
                 Token::new(String::from("float"), BASE_DATATYPE, PRIMITIVE_FLOAT, 3, 3),
                 Token::new(String::from("double"),
                            BASE_DATATYPE,
                            PRIMITIVE_DOUBLE,
                            4,
                            4),
                 Token::new(String::from("char"), BASE_DATATYPE, PRIMITIVE_CHAR, 5, 5),
                 Token::new(String::from("bool"), BASE_DATATYPE, PRIMITIVE_BOOL, 6, 6),
                 Token::new(String::from("void"), BASE_DATATYPE, PRIMITIVE_VOID, 7, 7),
                 Token::new(String::from("typedef"), BASE_NONE, PRIMITIVE_TYPEDEF, 8, 8)];
        assert_eq!(tok_vector, tok.tokenize());
    }

    #[test]
    fn test_tokenize_comments() {
        let text = read_file("test_cases/unit_tests/tokenize_comments.cpp");
        let mut tok = lexer::Tokenizer::new(&text);
        let tok_vector = vec![Token::new(String::from("//Hello World"),
                                         BASE_COMMENT,
                                         COMMENT_SINGLE,
                                         0,
                                         0),
                              Token::new(String::from("/** hello\n* ,\n* world\n*/"),
                                         BASE_COMMENT,
                                         COMMENT_MULTI,
                                         1,
                                         1),
                              Token::new(String::from("// Goodbye"),
                                         BASE_COMMENT,
                                         COMMENT_SINGLE,
                                         2,
                                         2)];
        assert_eq!(tok_vector, tok.tokenize());
    }

    #[test]
    fn test_tokenize_operators() {
        let text = read_file("test_cases/unit_tests/tokenize_operators.cpp");
        let mut tok = lexer::Tokenizer::new(&text);
        let tok_vector = vec![Token::new(String::from("++"), BASE_UNOP, OP_INC, 0, 0),
                              Token::new(String::from("--"), BASE_UNOP, OP_DEC, 1, 1),
                              Token::new(String::from("~"), BASE_UNOP, OP_BITNEG, 2, 2),
                              Token::new(String::from("!"), BASE_UNOP, OP_LOGNOT, 3, 3),

                              Token::new(String::from("+"), BASE_BINOP, OP_PLUS, 4, 4),
                              Token::new(String::from("-"), BASE_BINOP, OP_MINUS, 5, 5),
                              Token::new(String::from("/"), BASE_BINOP, OP_DIV, 6, 6),
                              Token::new(String::from("*"), BASE_BINOP, OP_MUL, 7, 7),
                              Token::new(String::from("%"), BASE_BINOP, OP_MOD, 8, 8),
                              Token::new(String::from(">"), BASE_BINOP, OP_GT, 9, 9),
                              Token::new(String::from(">="), BASE_BINOP, OP_GE, 10, 10),
                              Token::new(String::from(">>"), BASE_BINOP, OP_BITRSHIFT, 11, 11),
                              Token::new(String::from("<"), BASE_BINOP, OP_LT, 12, 12),
                              Token::new(String::from("<="), BASE_BINOP, OP_LE, 13, 13),
                              Token::new(String::from("<<"), BASE_BINOP, OP_BITLSHIFT, 14, 14),
                              Token::new(String::from("=="), BASE_BINOP, OP_EQU, 15, 15),
                              Token::new(String::from("!="), BASE_BINOP, OP_NEQ, 16, 16),
                              Token::new(String::from("&"), BASE_BINOP, OP_BITAND, 17, 17),
                              Token::new(String::from("&&"), BASE_BINOP, OP_LOGAND, 18, 18),
                              Token::new(String::from("|"), BASE_BINOP, OP_BITOR, 19, 19),
                              Token::new(String::from("||"), BASE_BINOP, OP_LOGOR, 20, 20),

                              Token::new(String::from("="), BASE_NONE, OP_ASSIGN, 21, 21),
                              Token::new(String::from("+="), BASE_ASSIGNOP, OP_PLUSEQU, 22, 22),
                              Token::new(String::from("-="), BASE_ASSIGNOP, OP_MINEQU, 23, 23),
                              Token::new(String::from("/="), BASE_ASSIGNOP, OP_DIVEQU, 24, 24),
                              Token::new(String::from("%="), BASE_ASSIGNOP, OP_MODEQU, 25, 25)];
        assert_eq!(tok_vector, tok.tokenize());
    }

    #[test]
    fn test_tokenize_punctuations() {
        let text = read_file("test_cases/unit_tests/tokenize_punctuations.cpp");
        let mut tok = lexer::Tokenizer::new(&text);
        let tok_vector = vec![Token::new(String::from("{"), BASE_NONE, LEFT_CBRACE, 0, 0),
                              Token::new(String::from("}"), BASE_NONE, RIGHT_CBRACE, 1, 1),
                              Token::new(String::from("("), BASE_NONE, LEFT_BRACKET, 2, 2),
                              Token::new(String::from(")"), BASE_NONE, RIGHT_BRACKET, 3, 3),
                              Token::new(String::from("["), BASE_NONE, LEFT_SBRACKET, 4, 4),
                              Token::new(String::from("]"), BASE_NONE, RIGHT_SBRACKET, 5, 5),
                              Token::new(String::from(":"), BASE_NONE, COLON, 6, 6),
                              Token::new(String::from(";"), BASE_NONE, SEMICOLON, 7, 7),
                              Token::new(String::from(","), BASE_NONE, COMMA, 8, 8)];
        assert_eq!(tok_vector, tok.tokenize());
    }

    #[test]
    fn test_tokenize_values() {
        let text = read_file("test_cases/unit_tests/tokenize_values.cpp");
        let mut tok = lexer::Tokenizer::new(&text);
        let tok_vector = vec![Token::new(String::from("\"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ`1234567890-=[]\\;\',./~!@#$%^&*()_+{}|:\\\"<>?\\\"\'\""),
                                         BASE_VALUE,
                                         STRING,
                                         0,
                                         0),
                              Token::new(String::from("'a'"), BASE_VALUE, CHAR_VAL, 1, 1),
                              Token::new(String::from("\'\\\'\'"), BASE_VALUE, CHAR_VAL, 2, 2),
                              Token::new(String::from("\'\\\"\'"), BASE_VALUE, CHAR_VAL, 3, 3),
                              Token::new(String::from("\'\\\\\'"), BASE_VALUE, CHAR_VAL, 4, 4),
                              Token::new(String::from("1234567890"), BASE_VALUE, NUM_INT, 5, 5),
                              Token::new(String::from("1234567890.0987654321"),
                                         BASE_VALUE,
                                         NUM_FLOAT,
                                         6,
                                         6),
                              Token::new(String::from("true"), BASE_VALUE, TRUE_VAL, 7, 7),
                              Token::new(String::from("false"), BASE_VALUE, FALSE_VAL, 8, 8)];
        assert_eq!(tok_vector, tok.tokenize());
    }

    #[test]
    fn test_tokenize_ids() {
        let text = read_file("test_cases/unit_tests/tokenize_ids.cpp");
        let mut tok = lexer::Tokenizer::new(&text);
        let tok_vector =
            vec![Token::new(String::from("_"), BASE_NONE, IDENTIFIER, 0, 0),
                 Token::new(String::from("_1123abcd_deff04"),
                            BASE_NONE,
                            IDENTIFIER,
                            1,
                            1),
                 Token::new(String::from("abcd_deff04_"), BASE_NONE, IDENTIFIER, 2, 2),
                 Token::new(String::from("inte"), BASE_NONE, IDENTIFIER, 3, 3),
                 Token::new(String::from("main"), BASE_NONE, MAIN, 4, 4)];
        assert_eq!(tok_vector, tok.tokenize());
    }
}
