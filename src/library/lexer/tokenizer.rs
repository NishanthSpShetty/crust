#![allow(dead_code)]

use std::str::Chars;

use library::lexeme::definition::{TokenKind, TokenType, BLACK_HOLE};
use library::lexeme::token::Token;
use library::lexer::helper::*;

pub struct Tokenizer<'a> {
    line_no: u32,
    id: u32,
    position: usize,
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
            position: 0,
            id: 0,
            line_no: 1,
            current_char: BLACK_HOLE,
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
                    self.current_char = self.get_next_char();
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
                    self.push_to_tok_buffer(TokenType::StringValue, TokenKind::Values);
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
                    self.push_to_tok_buffer(TokenType::CharValue, TokenKind::Values);
                }

                '{' => {
                    self.push_advance();
                    self.push_to_tok_buffer(TokenType::LeftCurlyBrace, TokenKind::SpecialChars);
                }

                '(' => {
                    self.push_advance();
                    self.push_to_tok_buffer(TokenType::LeftBracket, TokenKind::SpecialChars);
                }

                '[' => {
                    self.push_advance();
                    self.push_to_tok_buffer(TokenType::LeftSquareBracket, TokenKind::SpecialChars);
                }

                '}' => {
                    self.push_advance();
                    self.push_to_tok_buffer(TokenType::RightCurlyBrace, TokenKind::SpecialChars);
                }

                ')' => {
                    self.push_advance();
                    self.push_to_tok_buffer(TokenType::RightBracket, TokenKind::SpecialChars);
                }

                ']' => {
                    self.push_advance();
                    self.push_to_tok_buffer(TokenType::RightSquareBracket, TokenKind::SpecialChars);
                }

                '<' => {
                    self.push_advance();
                    match self.current_char {
                        '<' => {
                            self.push_advance();
                            self.push_to_tok_buffer(
                                TokenType::BitwiseLeftShift,
                                TokenKind::BinaryOperators,
                            );
                        }

                        '=' => {
                            self.push_advance();
                            self.push_to_tok_buffer(
                                TokenType::LessThanOrEqual,
                                TokenKind::BinaryOperators,
                            );
                        }

                        _ => {
                            self.push_to_tok_buffer(
                                TokenType::LessThan,
                                TokenKind::BinaryOperators,
                            );
                        }
                    }
                }

                '>' => {
                    self.push_advance();
                    match self.current_char {
                        '>' => {
                            self.push_advance();
                            self.push_to_tok_buffer(
                                TokenType::BitwiseRightShift,
                                TokenKind::BinaryOperators,
                            );
                        }

                        '=' => {
                            self.push_advance();
                            self.push_to_tok_buffer(
                                TokenType::GreaterThanOrEqual,
                                TokenKind::BinaryOperators,
                            );
                        }

                        _ => {
                            self.push_to_tok_buffer(
                                TokenType::GreaterThan,
                                TokenKind::BinaryOperators,
                            );
                        }
                    }
                }

                '=' => {
                    self.push_advance();
                    match self.current_char {
                        '=' => {
                            self.push_advance();
                            self.push_to_tok_buffer(TokenType::Equal, TokenKind::BinaryOperators);
                        }

                        _ => {
                            self.push_to_tok_buffer(
                                TokenType::Assignment,
                                TokenKind::AssignmentOperators,
                            );
                        }
                    }
                }

                '_' | 'a'..='z' | 'A'..='Z' => {
                    self.push_advance();
                    loop {
                        match self.current_char {
                            '_' | 'a'..='z' | 'A'..='Z' | '0'..='9' => {
                                self.push_advance();
                            }
                            _ => {
                                break;
                            }
                        }
                    }
                    let (token_type, base_type) = identify_token_type(&self.token);
                    self.push_to_tok_buffer(token_type, base_type);
                }

                '0'..='9' => {
                    self.push_advance();
                    let mut is_int: bool = true;

                    loop {
                        match self.current_char {
                            '0'..='9' => {
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
                        self.push_to_tok_buffer(TokenType::NumberInteger, TokenKind::Values);
                    } else {
                        self.push_to_tok_buffer(TokenType::NumberFloat, TokenKind::Values);
                    }
                }

                '+' => {
                    self.push_advance();
                    match self.current_char {
                        '+' => {
                            self.push_advance();
                            self.push_to_tok_buffer(
                                TokenType::Increment,
                                TokenKind::UnaryOperators,
                            );
                        }

                        '=' => {
                            self.push_advance();
                            self.push_to_tok_buffer(
                                TokenType::PlusEqual,
                                TokenKind::AssignmentOperators,
                            );
                        }
                        _ => {
                            self.push_to_tok_buffer(TokenType::Plus, TokenKind::BinaryOperators);
                        }
                    };
                }

                '-' => {
                    self.push_advance();
                    match self.current_char {
                        '-' => {
                            self.push_advance();
                            self.push_to_tok_buffer(
                                TokenType::Decrement,
                                TokenKind::UnaryOperators,
                            );
                        }

                        '=' => {
                            self.push_advance();
                            self.push_to_tok_buffer(
                                TokenType::MinusEqual,
                                TokenKind::AssignmentOperators,
                            );
                        }
                        '>' => {
                            self.push_advance();
                            self.push_to_tok_buffer(TokenType::Arrow, TokenKind::SpecialChars);
                        }
                        _ => {
                            self.push_to_tok_buffer(TokenType::Minus, TokenKind::BinaryOperators);
                        }
                    };
                }

                '*' => {
                    self.push_advance();
                    match self.current_char {
                        '=' => {
                            self.push_advance();
                            self.push_to_tok_buffer(
                                TokenType::MultiplyEqual,
                                TokenKind::AssignmentOperators,
                            );
                        }
                        _ => {
                            self.push_to_tok_buffer(
                                TokenType::Multiplication,
                                TokenKind::BinaryOperators,
                            );
                        }
                    };
                }

                '%' => {
                    self.push_advance();
                    match self.current_char {
                        '=' => {
                            self.push_advance();
                            self.push_to_tok_buffer(
                                TokenType::ModuleEqual,
                                TokenKind::AssignmentOperators,
                            );
                        }
                        _ => {
                            self.push_to_tok_buffer(TokenType::Module, TokenKind::BinaryOperators);
                        }
                    };
                }

                '~' => {
                    self.push_advance();
                    self.push_to_tok_buffer(TokenType::BitwiseNegate, TokenKind::UnaryOperators);
                }

                // could be address or bitwise operator
                // TODO : how to find address off operator?
                //      If left side expression is not rvalue then it must be address off,
                '&' => {
                    self.push_advance();
                    match self.current_char {
                        '&' => {
                            self.push_advance();
                            self.push_to_tok_buffer(
                                TokenType::LogicalAnd,
                                TokenKind::BinaryOperators,
                            );
                        }
                        _ => {
                            //this is ambiguous resolution, should be validated at the parser side,
                            //tokenizer has very limited knowledge of the expression
                            self.push_to_tok_buffer(
                                TokenType::BitwiseAnd,
                                TokenKind::BinaryOperators,
                            );
                        }
                    };
                }

                '|' => {
                    self.push_advance();
                    match self.current_char {
                        '|' => {
                            self.push_advance();
                            self.push_to_tok_buffer(
                                TokenType::LogicalOr,
                                TokenKind::BinaryOperators,
                            );
                        }
                        _ => {
                            self.push_to_tok_buffer(
                                TokenType::BitwiseOr,
                                TokenKind::BinaryOperators,
                            );
                        }
                    };
                }

                '!' => {
                    self.push_advance();
                    match self.current_char {
                        '=' => {
                            self.push_advance();
                            self.push_to_tok_buffer(
                                TokenType::NotEqual,
                                TokenKind::BinaryOperators,
                            );
                        }
                        _ => {
                            self.push_to_tok_buffer(
                                TokenType::LogicalNot,
                                TokenKind::UnaryOperators,
                            );
                        }
                    };
                }

                '/' => {
                    self.push_advance();
                    match self.current_char {
                        '*' => {
                            // start of multi line comment
                            let mut line_num = self.line_no;

                            loop {
                                self.push_advance();

                                if self.current_char == '\n' {
                                    //count the new lines
                                    line_num += 1;
                                } else if self.current_char == '*' {
                                    self.push_advance();
                                    if self.current_char == '/' {
                                        self.push_advance();
                                        self.push_to_tok_buffer(
                                            TokenType::MultilineComment,
                                            TokenKind::Comments,
                                        );
                                        self.line_no = line_num;
                                        break;
                                    } else if self.current_char == '\n' {
                                        //count the new lines
                                        line_num += 1;
                                    }
                                }
                            }
                        }

                        '/' => {
                            // single line comment
                            loop {
                                match self.current_char {
                                    '\n' | '\r' | '\0' => {
                                        self.push_to_tok_buffer(
                                            TokenType::SingleLineComment,
                                            TokenKind::Comments,
                                        );
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
                            self.push_to_tok_buffer(
                                TokenType::DivideEqual,
                                TokenKind::AssignmentOperators,
                            );
                        }

                        _ => {
                            self.push_to_tok_buffer(TokenType::Divide, TokenKind::BinaryOperators);
                        }
                    };
                }

                ';' => {
                    self.push_advance();
                    self.push_to_tok_buffer(TokenType::Semicolon, TokenKind::SpecialChars);
                }

                ':' => {
                    self.push_advance();
                    match self.current_char {
                        ':' => {
                            self.push_advance();
                            self.push_to_tok_buffer(
                                TokenType::ScopeResolution,
                                TokenKind::SpecialChars,
                            );
                        }
                        _ => {
                            self.push_to_tok_buffer(TokenType::Colon, TokenKind::SpecialChars);
                        }
                    };
                }

                ',' => {
                    self.push_advance();
                    self.push_to_tok_buffer(TokenType::Comma, TokenKind::SpecialChars);
                }
                '#' => {
                    self.push_advance();
                    loop {
                        match self.current_char {
                            '_' | 'a'..='z' | 'A'..='Z' | '0'..='9' => {
                                self.push_advance();
                            }
                            _ => {
                                break;
                            }
                        }
                    }
                    let (token_type, base_type) = identify_token_type(&self.token);

                    //read in entire line as the `token_type`.
                    loop {
                        match self.current_char {
                            '\n' => {
                                break;
                            }
                            _ => {
                                self.push_advance();
                            }
                        }
                    }
                    self.push_to_tok_buffer(token_type, base_type);
                }
                '?' => {
                    self.push_advance();
                    self.push_to_tok_buffer(TokenType::TernaryOpetator, TokenKind::BinaryOperators);
                }

                _ => {
                    self.push_advance();
                    self.push_to_tok_buffer(TokenType::Others, TokenKind::None);
                }
            };

            if self.position > self.length {
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
        self.position += 1;
        if let Some(ch) = self.input.next() {
            ch
        } else {
            '\0'
        }
    }

    // Push the token read so far into Token buffer
    // and clear the token
    fn push_to_tok_buffer(&mut self, token_type: TokenType, token_kind: TokenKind) {
        let token: String = self.token.iter().cloned().collect();
        if !token.is_empty() {
            let token = Token::new(token, token_kind, token_type, self.line_no, self.id);
            self.token_buffer.push(token);
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
        self.position -= 1;
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
    // use std::io::Write;
    use std::io::BufReader;
    use std::io::Read;

    use library::lexeme::definition::{TokenKind, TokenType};
    use library::lexeme::token::Token;
    use library::lexer::tokenizer::Tokenizer;

    fn read_file(path: &str) -> String {
        let file = match File::open(path) {
            Ok(f) => f,
            Err(..) => panic!("Unable to open input source file."),
        };
        let mut reader = BufReader::new(&file);
        let mut text: String = String::new();
        reader
            .read_to_string(&mut text)
            .expect("Failed to read file content into text buffer.");
        text
    }

    #[test]
    fn test_get_next_char() {
        let get_next_char = |x: &str| Tokenizer::new(&x).get_next_char();

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
    fn test_that_tokenizer_created_successfully() {
        let tok = Tokenizer::new("cout << \"Hello World\"");
        // pos
        assert_eq!(0, tok.position);
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
        let mut tok = Tokenizer::new("a=\"2\"");
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
        assert_eq!(tok.token, ['a', '=', '\"', '2', '\"']);
        assert_eq!(tok.current_char, '\0');
    }

    #[test]
    fn test_push_to_tok_buffer() {
        let mut tok = Tokenizer::new("a=\"2\"");

        // set tok.current_char from tok.get_next_char()
        // do push_advance()
        // do push_to_tok_buffer()
        // check if tok.token is empty
        // check if tok.token_buffer has current token
        tok.current_char = tok.get_next_char();

        tok.push_advance();
        tok.push_to_tok_buffer(TokenType::Identifier, TokenKind::Identifiers);
        assert_eq!(tok.token_buffer[0].get_token_type(), TokenType::Identifier);
        assert_eq!(tok.token_buffer[0].get_token_kind(), TokenKind::Identifiers);
        assert_eq!(tok.token_buffer[0].get_token_value(), String::from("a"));
        assert_eq!(tok.token_buffer[0].get_token_line_num(), 1);
        assert_eq!(0, tok.token.len());

        tok.push_advance();
        tok.push_to_tok_buffer(TokenType::Assignment, TokenKind::AssignmentOperators);
        assert_eq!(tok.token_buffer[1].get_token_type(), TokenType::Assignment);
        assert_eq!(
            tok.token_buffer[1].get_token_kind(),
            TokenKind::AssignmentOperators
        );
        assert_eq!(tok.token_buffer[1].get_token_value(), String::from("="));
        assert_eq!(tok.token_buffer[1].get_token_line_num(), 1);

        tok.push_advance();
        tok.push_advance();
        tok.push_advance();
        tok.push_to_tok_buffer(TokenType::StringValue, TokenKind::Values);
        assert_eq!(tok.token_buffer[2].get_token_type(), TokenType::StringValue);
        assert_eq!(tok.token_buffer[2].get_token_kind(), TokenKind::Values);
        assert_eq!(tok.token_buffer[2].get_token_value(), String::from("\"2\""));
        assert_eq!(tok.token_buffer[2].get_token_line_num(), 1);
        assert_eq!(0, tok.token.len());
    }

    #[test]
    fn test_tokenize_keywords() {
        let text = read_file("src/test/resources/tokenize_keywords.cpp");
        let mut tok = Tokenizer::new(&text);
        let expected = vec![
            Token::new(
                String::from("signed"),
                TokenKind::Modifiers,
                TokenType::Signed,
                1,
                0,
            ),
            Token::new(
                String::from("unsigned"),
                TokenKind::Modifiers,
                TokenType::Unsigned,
                2,
                1,
            ),
            Token::new(
                String::from("class"),
                TokenKind::Keyword,
                TokenType::KeywordClass,
                3,
                2,
            ),
            Token::new(
                String::from("new"),
                TokenKind::Keyword,
                TokenType::KeywordNew,
                4,
                3,
            ),
            Token::new(
                String::from("while"),
                TokenKind::Keyword,
                TokenType::KeywordWhile,
                5,
                4,
            ),
            Token::new(
                String::from("for"),
                TokenKind::Keyword,
                TokenType::KeywordFor,
                6,
                5,
            ),
            Token::new(
                String::from("do"),
                TokenKind::Keyword,
                TokenType::KeywordDo,
                7,
                6,
            ),
            Token::new(
                String::from("break"),
                TokenKind::Keyword,
                TokenType::KeywordBreak,
                8,
                7,
            ),
            Token::new(
                String::from("continue"),
                TokenKind::Keyword,
                TokenType::KeywordContinue,
                9,
                8,
            ),
            Token::new(
                String::from("switch"),
                TokenKind::Keyword,
                TokenType::KeywordSwitch,
                10,
                9,
            ),
            Token::new(
                String::from("if"),
                TokenKind::Keyword,
                TokenType::KeywordIf,
                11,
                10,
            ),
            Token::new(
                String::from("else"),
                TokenKind::Keyword,
                TokenType::KeywordElse,
                12,
                11,
            ),
            Token::new(
                String::from("public"),
                TokenKind::Modifiers,
                TokenType::KeywordPublic,
                13,
                12,
            ),
            Token::new(
                String::from("private"),
                TokenKind::Modifiers,
                TokenType::keywordPrivate,
                14,
                13,
            ),
            Token::new(
                String::from("protected"),
                TokenKind::Modifiers,
                TokenType::KeywordProtected,
                15,
                14,
            ),
            Token::new(
                String::from("case"),
                TokenKind::Keyword,
                TokenType::KeywordCase,
                16,
                15,
            ),
            Token::new(
                String::from("static"),
                TokenKind::Modifiers,
                TokenType::KeywordStatic,
                17,
                16,
            ),
            Token::new(
                String::from("const"),
                TokenKind::Keyword,
                TokenType::KeywordConst,
                18,
                17,
            ),
            Token::new(
                String::from("default"),
                TokenKind::Keyword,
                TokenType::KeywordDefault,
                19,
                18,
            ),
            Token::new(
                String::from("return"),
                TokenKind::Keyword,
                TokenType::KeywordReturn,
                20,
                19,
            ),
        ];
        assert_eq!(expected, tok.tokenize());
    }

    #[test]
    fn test_tokenize_types() {
        let text = read_file("src/test/resources/tokenize_types.cpp");
        let mut tok = Tokenizer::new(&text);
        let expected = vec![
            Token::new(
                String::from("int"),
                TokenKind::DataTypes,
                TokenType::Integer,
                1,
                0,
            ),
            Token::new(
                String::from("short"),
                TokenKind::DataTypes,
                TokenType::Short,
                2,
                1,
            ),
            Token::new(
                String::from("long"),
                TokenKind::DataTypes,
                TokenType::Long,
                3,
                2,
            ),
            Token::new(
                String::from("float"),
                TokenKind::DataTypes,
                TokenType::Float,
                4,
                3,
            ),
            Token::new(
                String::from("double"),
                TokenKind::DataTypes,
                TokenType::Double,
                5,
                4,
            ),
            Token::new(
                String::from("char"),
                TokenKind::DataTypes,
                TokenType::Character,
                6,
                5,
            ),
            Token::new(
                String::from("bool"),
                TokenKind::DataTypes,
                TokenType::Boolean,
                7,
                6,
            ),
            Token::new(
                String::from("void"),
                TokenKind::DataTypes,
                TokenType::Void,
                8,
                7,
            ),
            Token::new(
                String::from("typedef"),
                TokenKind::Typedef,
                TokenType::Typedef,
                9,
                8,
            ),
        ];
        assert_eq!(expected, tok.tokenize());
    }

    #[test]
    fn test_tokenize_comments() {
        let text = read_file("src/test/resources/tokenize_comments.cpp");
        let mut tok = Tokenizer::new(&text);
        let expected = vec![
            Token::new(
                String::from("//Hello World"),
                TokenKind::Comments,
                TokenType::SingleLineComment,
                1,
                0,
            ),
            Token::new(
                String::from("/** hello\n * world\n * this is multi\n * line comment\n * Do not modify the code below, i have no\n * idea what it does, but needed very much\n */"),
                TokenKind::Comments,
                TokenType::MultilineComment,
                2,
                1,
            ),
            Token::new(
                String::from("// Goodbye"),
                TokenKind::Comments,
                TokenType::SingleLineComment,
                9,
                2,
            ),
        ];
        let result = tok.tokenize();
        let mut index: usize = 0;
        while index < expected.len() {
            assert_eq!(
                expected[index], result[index],
                "\nTest failed for index {}",
                index
            );
            index += 1;
        }
    }

    #[test]
    fn test_tokenize_operators() {
        let text = read_file("src/test/resources/tokenize_operators.cpp");
        let mut tok = Tokenizer::new(&text);
        let expected = vec![
            Token::new(
                String::from("++"),
                TokenKind::UnaryOperators,
                TokenType::Increment,
                1,
                0,
            ),
            Token::new(
                String::from("--"),
                TokenKind::UnaryOperators,
                TokenType::Decrement,
                2,
                1,
            ),
            Token::new(
                String::from("~"),
                TokenKind::UnaryOperators,
                TokenType::BitwiseNegate,
                3,
                2,
            ),
            Token::new(
                String::from("!"),
                TokenKind::UnaryOperators,
                TokenType::LogicalNot,
                4,
                3,
            ),
            Token::new(
                String::from("+"),
                TokenKind::BinaryOperators,
                TokenType::Plus,
                5,
                4,
            ),
            Token::new(
                String::from("-"),
                TokenKind::BinaryOperators,
                TokenType::Minus,
                6,
                5,
            ),
            Token::new(
                String::from("/"),
                TokenKind::BinaryOperators,
                TokenType::Divide,
                7,
                6,
            ),
            Token::new(
                String::from("*"),
                TokenKind::BinaryOperators,
                TokenType::Multiplication,
                8,
                7,
            ),
            Token::new(
                String::from("%"),
                TokenKind::BinaryOperators,
                TokenType::Module,
                9,
                8,
            ),
            Token::new(
                String::from(">"),
                TokenKind::BinaryOperators,
                TokenType::GreaterThan,
                10,
                9,
            ),
            Token::new(
                String::from(">="),
                TokenKind::BinaryOperators,
                TokenType::GreaterThanOrEqual,
                11,
                10,
            ),
            Token::new(
                String::from(">>"),
                TokenKind::BinaryOperators,
                TokenType::BitwiseRightShift,
                12,
                11,
            ),
            Token::new(
                String::from("<"),
                TokenKind::BinaryOperators,
                TokenType::LessThan,
                13,
                12,
            ),
            Token::new(
                String::from("<="),
                TokenKind::BinaryOperators,
                TokenType::LessThanOrEqual,
                14,
                13,
            ),
            Token::new(
                String::from("<<"),
                TokenKind::BinaryOperators,
                TokenType::BitwiseLeftShift,
                15,
                14,
            ),
            Token::new(
                String::from("=="),
                TokenKind::BinaryOperators,
                TokenType::Equal,
                16,
                15,
            ),
            Token::new(
                String::from("!="),
                TokenKind::BinaryOperators,
                TokenType::NotEqual,
                17,
                16,
            ),
            Token::new(
                String::from("&"),
                TokenKind::BinaryOperators,
                TokenType::BitwiseAnd,
                18,
                17,
            ),
            Token::new(
                String::from("&&"),
                TokenKind::BinaryOperators,
                TokenType::LogicalAnd,
                19,
                18,
            ),
            Token::new(
                String::from("|"),
                TokenKind::BinaryOperators,
                TokenType::BitwiseOr,
                20,
                19,
            ),
            Token::new(
                String::from("||"),
                TokenKind::BinaryOperators,
                TokenType::LogicalOr,
                21,
                20,
            ),
            Token::new(
                String::from("="),
                TokenKind::AssignmentOperators,
                TokenType::Assignment,
                22,
                21,
            ),
            Token::new(
                String::from("+="),
                TokenKind::AssignmentOperators,
                TokenType::PlusEqual,
                23,
                22,
            ),
            Token::new(
                String::from("-="),
                TokenKind::AssignmentOperators,
                TokenType::MinusEqual,
                24,
                23,
            ),
            Token::new(
                String::from("/="),
                TokenKind::AssignmentOperators,
                TokenType::DivideEqual,
                25,
                24,
            ),
            Token::new(
                String::from("%="),
                TokenKind::AssignmentOperators,
                TokenType::ModuleEqual,
                26,
                25,
            ),
            Token::new(
                String::from("->"),
                TokenKind::SpecialChars,
                TokenType::Arrow,
                27,
                26,
            ),
            Token::new(
                String::from("::"),
                TokenKind::SpecialChars,
                TokenType::ScopeResolution,
                28,
                27,
            ),
            Token::new(
                String::from("?"),
                TokenKind::BinaryOperators,
                TokenType::TernaryOpetator,
                29,
                28,
            ),
        ];
        let result = tok.tokenize();
        let mut index = 0;
        while index < expected.len() {
            assert_eq!(
                expected[index], result[index],
                "\nTest failed for index {}",
                index
            );
            index += 1;
        }
    }

    #[test]
    fn test_tokenize_punctuations() {
        let text = read_file("src/test/resources/tokenize_punctuations.cpp");
        let mut tok = Tokenizer::new(&text);
        let expected = vec![
            Token::new(
                String::from("{"),
                TokenKind::SpecialChars,
                TokenType::LeftCurlyBrace,
                1,
                0,
            ),
            Token::new(
                String::from("}"),
                TokenKind::SpecialChars,
                TokenType::RightCurlyBrace,
                2,
                1,
            ),
            Token::new(
                String::from("("),
                TokenKind::SpecialChars,
                TokenType::LeftBracket,
                3,
                2,
            ),
            Token::new(
                String::from(")"),
                TokenKind::SpecialChars,
                TokenType::RightBracket,
                4,
                3,
            ),
            Token::new(
                String::from("["),
                TokenKind::SpecialChars,
                TokenType::LeftSquareBracket,
                5,
                4,
            ),
            Token::new(
                String::from("]"),
                TokenKind::SpecialChars,
                TokenType::RightSquareBracket,
                6,
                5,
            ),
            Token::new(
                String::from(":"),
                TokenKind::SpecialChars,
                TokenType::Colon,
                7,
                6,
            ),
            Token::new(
                String::from(";"),
                TokenKind::SpecialChars,
                TokenType::Semicolon,
                8,
                7,
            ),
            Token::new(
                String::from(","),
                TokenKind::SpecialChars,
                TokenType::Comma,
                9,
                8,
            ),
        ];
        let result = tok.tokenize();
        let mut index = 0;
        while index < expected.len() {
            assert_eq!(
                expected[index], result[index],
                "\nTest failed for index {}",
                index
            );
            index += 1;
        }
    }

    #[test]
    fn test_tokenize_values() {
        let text = read_file("src/test/resources/tokenize_values.cpp");
        let mut tok = Tokenizer::new(&text);
        let expected = vec![
            Token::new(String::from("\"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ`1234567890-=[]\\;\',./~!@#$%^&*()_+{}|:\\\"<>?\\\"\'\""),
                       TokenKind::Values,
                       TokenType::StringValue,
                       1,
                       0),
            Token::new(String::from("'a'"), TokenKind::Values, TokenType::CharValue, 2, 1),
            Token::new(String::from("\'\\\'\'"), TokenKind::Values, TokenType::CharValue, 3, 2),
            Token::new(String::from("\'\\\"\'"), TokenKind::Values, TokenType::CharValue, 4, 3),
            Token::new(String::from("\'\\\\\'"), TokenKind::Values, TokenType::CharValue, 5, 4),
            Token::new(String::from("1234567890"), TokenKind::Values, TokenType::NumberInteger, 6, 5),
            Token::new(String::from("1234567890.0987654321"),
                       TokenKind::Values,
                       TokenType::NumberFloat,
                       7,
                       6),
            Token::new(String::from("true"), TokenKind::Values, TokenType::True, 8, 7),
            Token::new(String::from("false"), TokenKind::Values, TokenType::False, 9, 8)];
        let result = tok.tokenize();
        let mut index = 0;
        while index < expected.len() {
            assert_eq!(
                expected[index], result[index],
                "\nTest failed for index {}",
                index
            );
            index += 1;
        }
    }

    #[test]
    fn test_tokenize_ids() {
        let text = read_file("src/test/resources/tokenize_ids.cpp");
        let mut tok = Tokenizer::new(&text);
        let expected = vec![
            Token::new(
                String::from("_"),
                TokenKind::Identifiers,
                TokenType::Identifier,
                1,
                0,
            ),
            Token::new(
                String::from("_1123abcd_deff04"),
                TokenKind::Identifiers,
                TokenType::Identifier,
                2,
                1,
            ),
            Token::new(
                String::from("abcd_deff04_"),
                TokenKind::Identifiers,
                TokenType::Identifier,
                3,
                2,
            ),
            Token::new(
                String::from("integer"),
                TokenKind::Identifiers,
                TokenType::Identifier,
                4,
                3,
            ),
            Token::new(
                String::from("main"),
                TokenKind::Identifiers,
                TokenType::Main,
                5,
                4,
            ),
        ];
        let result = tok.tokenize();

        let mut index = 0;
        while index < expected.len() {
            assert_eq!(
                expected[index], result[index],
                "\nTest failed for index {}",
                index
            );
            index += 1;
        }
    }
}
