use std::str::Chars;

use crate::library::lexeme::definition::{TokenKind, TokenType, BLACK_HOLE};
use crate::library::lexeme::token::Token;
use crate::library::lexer::helper::*;

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
    pub fn new(text: &str) -> Tokenizer {
        let token_vec: Vec<char> = Vec::new();
        let token_stream: Vec<Token> = Vec::new();

        Tokenizer {
            position: 0,
            id: 0,
            line_no: 0,
            current_char: BLACK_HOLE,
            length: text.len(),
            token: token_vec,
            token_buffer: token_stream,
            input: text.chars(),
        }
    }

    ///increment the line number
    fn new_line(&mut self) {
        self.line_no += 1;
    }

    /// Walks over given text and returns the stream of tokens
    /// We will take ownership of Tokenizer, as we wont be needing after.
    pub fn tokenize(mut self) -> Vec<Token> {
        self.current_char = self.get_next_char();
        loop {
            match self.current_char {
                '\n' => {
                    self.new_line();
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

                    while let '_' | 'a'..='z' | 'A'..='Z' | '0'..='9' = self.current_char {
                        self.push_advance();
                    }

                    //                    loop {
                    //                        match self.current_char {
                    //                            '_' | 'a'..='z' | 'A'..='Z' | '0'..='9' => {
                    //                                self.push_advance();
                    //                            }
                    //                            _ => {
                    //                                break;
                    //                            }
                    //                        }
                    //                    }
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
                            //FIXME: could be address or bitwise operator
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
                            loop {
                                self.push_advance();
                                if self.current_char == '*' {
                                    self.push_advance();
                                    if self.current_char == '/' {
                                        self.push_advance();
                                        self.push_to_tok_buffer(
                                            TokenType::MultilineComment,
                                            TokenKind::Comments,
                                        );
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
                                        self.push_to_tok_buffer(
                                            TokenType::SingleLineComment,
                                            TokenKind::Comments,
                                        );
                                        self.current_char = self.get_next_char();
                                        self.new_line();
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
                    self.push_to_tok_buffer(TokenType::HeaderInclude, TokenKind::Preprocessors);
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

        //as we own the self, we can return the ownership of token_buffer to the caller.
        //All we want from here is the tokens
        self.token_buffer
    }

    /// Returns the next char in a input stream pointed by `pos` position
    /// null (\0) otherwise
    fn get_next_char(&mut self) -> char {
        self.position += 1;
        if let Some(ch) = self.input.next() {
            ch
        } else {
            '\0'
        }
    }

    /// Creates a Token from current token and pushes into Token buffer.
    /// Clear the current token
    fn push_to_tok_buffer(&mut self, token_type: TokenType, token_kind: TokenKind) {
        let token: String = self.token.iter().cloned().collect();
        if !token.is_empty() {
            let token = Token::new(token, token_kind, token_type, self.line_no, self.id);
            self.token_buffer.push(token);
            self.id += 1;
        }
        self.token.clear();
    }

    /// Push and Advance
    /// pushes the current character into self.token
    /// updates the current_char with next character
    fn push_advance(&mut self) {
        self.token.push(self.current_char);
        self.current_char = self.get_next_char();
    }
}

#[cfg(test)]
mod test {
    use super::Tokenizer;
    use crate::library::lexeme::{definition::TokenKind, definition::TokenType, token::Token};

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
        assert_eq!(tok.token_buffer[0].get_token_line_num(), 0);
        assert_eq!(0, tok.token.len());

        tok.push_advance();
        tok.push_to_tok_buffer(TokenType::Assignment, TokenKind::AssignmentOperators);
        assert_eq!(tok.token_buffer[1].get_token_type(), TokenType::Assignment);
        assert_eq!(
            tok.token_buffer[1].get_token_kind(),
            TokenKind::AssignmentOperators
        );
        assert_eq!(tok.token_buffer[1].get_token_value(), String::from("="));
        assert_eq!(tok.token_buffer[1].get_token_line_num(), 0);

        tok.push_advance();
        tok.push_advance();
        tok.push_advance();
        tok.push_to_tok_buffer(TokenType::StringValue, TokenKind::Values);
        assert_eq!(tok.token_buffer[2].get_token_type(), TokenType::StringValue);
        assert_eq!(tok.token_buffer[2].get_token_kind(), TokenKind::Values);
        assert_eq!(tok.token_buffer[2].get_token_value(), String::from("\"2\""));
        assert_eq!(tok.token_buffer[2].get_token_line_num(), 0);
        assert_eq!(0, tok.token.len());
    }

    #[test]
    fn test_tokenize_keywords() {
        let text ="signed\n unsigned\n class\n new\n while\n for\n do\n break\n continue\n switch\n if\n else\n public\n private\n protected\n case\n static\n const\n default\n return\n";
        let tok = Tokenizer::new(&text);
        let tok_vector = vec![
            Token::new(
                String::from("signed"),
                TokenKind::Modifiers,
                TokenType::Signed,
                0,
                0,
            ),
            Token::new(
                String::from("unsigned"),
                TokenKind::Modifiers,
                TokenType::Unsigned,
                1,
                1,
            ),
            Token::new(
                String::from("class"),
                TokenKind::Keyword,
                TokenType::KeywordClass,
                2,
                2,
            ),
            Token::new(
                String::from("new"),
                TokenKind::Keyword,
                TokenType::KeywordNew,
                3,
                3,
            ),
            Token::new(
                String::from("while"),
                TokenKind::Keyword,
                TokenType::KeywordWhile,
                4,
                4,
            ),
            Token::new(
                String::from("for"),
                TokenKind::Keyword,
                TokenType::KeywordFor,
                5,
                5,
            ),
            Token::new(
                String::from("do"),
                TokenKind::Keyword,
                TokenType::KeywordDo,
                6,
                6,
            ),
            Token::new(
                String::from("break"),
                TokenKind::Keyword,
                TokenType::KeywordBreak,
                7,
                7,
            ),
            Token::new(
                String::from("continue"),
                TokenKind::Keyword,
                TokenType::KeywordContinue,
                8,
                8,
            ),
            Token::new(
                String::from("switch"),
                TokenKind::Keyword,
                TokenType::KeywordSwitch,
                9,
                9,
            ),
            Token::new(
                String::from("if"),
                TokenKind::Keyword,
                TokenType::KeywordIf,
                10,
                10,
            ),
            Token::new(
                String::from("else"),
                TokenKind::Keyword,
                TokenType::KeywordElse,
                11,
                11,
            ),
            Token::new(
                String::from("public"),
                TokenKind::Modifiers,
                TokenType::KeywordPublic,
                12,
                12,
            ),
            Token::new(
                String::from("private"),
                TokenKind::Modifiers,
                TokenType::keywordPrivate,
                13,
                13,
            ),
            Token::new(
                String::from("protected"),
                TokenKind::Modifiers,
                TokenType::KeywordProtected,
                14,
                14,
            ),
            Token::new(
                String::from("case"),
                TokenKind::Keyword,
                TokenType::KeywordCase,
                15,
                15,
            ),
            Token::new(
                String::from("static"),
                TokenKind::Modifiers,
                TokenType::KeywordStatic,
                16,
                16,
            ),
            Token::new(
                String::from("const"),
                TokenKind::Modifiers,
                TokenType::KeywordConst,
                17,
                17,
            ),
            Token::new(
                String::from("default"),
                TokenKind::Keyword,
                TokenType::KeywordDefault,
                18,
                18,
            ),
            Token::new(
                String::from("return"),
                TokenKind::Keyword,
                TokenType::KeywordReturn,
                19,
                19,
            ),
        ];
        assert_eq!(tok_vector, tok.tokenize());
    }

    #[test]
    fn test_tokenize_types() {
        let text = "int\n short\n long\n float\n double\n char\n bool\n void\n typedef\n";
        let tok = Tokenizer::new(&text);
        let tok_vector = vec![
            Token::new(
                String::from("int"),
                TokenKind::DataTypes,
                TokenType::Integer,
                0,
                0,
            ),
            Token::new(
                String::from("short"),
                TokenKind::DataTypes,
                TokenType::Short,
                1,
                1,
            ),
            Token::new(
                String::from("long"),
                TokenKind::DataTypes,
                TokenType::Long,
                2,
                2,
            ),
            Token::new(
                String::from("float"),
                TokenKind::DataTypes,
                TokenType::Float,
                3,
                3,
            ),
            Token::new(
                String::from("double"),
                TokenKind::DataTypes,
                TokenType::Double,
                4,
                4,
            ),
            Token::new(
                String::from("char"),
                TokenKind::DataTypes,
                TokenType::Character,
                5,
                5,
            ),
            Token::new(
                String::from("bool"),
                TokenKind::DataTypes,
                TokenType::Boolean,
                6,
                6,
            ),
            Token::new(
                String::from("void"),
                TokenKind::DataTypes,
                TokenType::Void,
                7,
                7,
            ),
            Token::new(
                String::from("typedef"),
                TokenKind::Typedef,
                TokenType::Typedef,
                8,
                8,
            ),
        ];
        assert_eq!(tok_vector, tok.tokenize());
    }

    #[test]
    fn test_tokenize_comments() {
        let text = "// Hello World\n/** hello\n * world\n */\n// Goodbye";
        //read_file("src/test/resources/tokenize_comments.cpp");
        let tok = Tokenizer::new(&text);
        let tok_vector = vec![
            Token::new(
                String::from("// Hello World"),
                TokenKind::Comments,
                TokenType::SingleLineComment,
                0,
                0,
            ),
            Token::new(
                String::from("/** hello\n * world\n */"),
                TokenKind::Comments,
                TokenType::MultilineComment,
                1,
                1,
            ),
            Token::new(
                String::from("// Goodbye"),
                TokenKind::Comments,
                TokenType::SingleLineComment,
                2,
                2,
            ),
        ];
        assert_eq!(tok_vector, tok.tokenize(), "returns token of comments");
    }

    #[test]
    fn test_tokenize_operators() {
        let text = "++\n --\n ~\n !\n +\n -\n /\n *\n %\n >\n >=\n >>\n <\n <=\n <<\n ==\n !=\n &\n &&\n |\n ||\n =\n +=\n -=\n /=\n %=\n ->\n ::\n ?\n";
        let tok = Tokenizer::new(&text);
        let tok_vector = vec![
            Token::new(
                String::from("++"),
                TokenKind::UnaryOperators,
                TokenType::Increment,
                0,
                0,
            ),
            Token::new(
                String::from("--"),
                TokenKind::UnaryOperators,
                TokenType::Decrement,
                1,
                1,
            ),
            Token::new(
                String::from("~"),
                TokenKind::UnaryOperators,
                TokenType::BitwiseNegate,
                2,
                2,
            ),
            Token::new(
                String::from("!"),
                TokenKind::UnaryOperators,
                TokenType::LogicalNot,
                3,
                3,
            ),
            Token::new(
                String::from("+"),
                TokenKind::BinaryOperators,
                TokenType::Plus,
                4,
                4,
            ),
            Token::new(
                String::from("-"),
                TokenKind::BinaryOperators,
                TokenType::Minus,
                5,
                5,
            ),
            Token::new(
                String::from("/"),
                TokenKind::BinaryOperators,
                TokenType::Divide,
                6,
                6,
            ),
            Token::new(
                String::from("*"),
                TokenKind::BinaryOperators,
                TokenType::Multiplication,
                7,
                7,
            ),
            Token::new(
                String::from("%"),
                TokenKind::BinaryOperators,
                TokenType::Module,
                8,
                8,
            ),
            Token::new(
                String::from(">"),
                TokenKind::BinaryOperators,
                TokenType::GreaterThan,
                9,
                9,
            ),
            Token::new(
                String::from(">="),
                TokenKind::BinaryOperators,
                TokenType::GreaterThanOrEqual,
                10,
                10,
            ),
            Token::new(
                String::from(">>"),
                TokenKind::BinaryOperators,
                TokenType::BitwiseRightShift,
                11,
                11,
            ),
            Token::new(
                String::from("<"),
                TokenKind::BinaryOperators,
                TokenType::LessThan,
                12,
                12,
            ),
            Token::new(
                String::from("<="),
                TokenKind::BinaryOperators,
                TokenType::LessThanOrEqual,
                13,
                13,
            ),
            Token::new(
                String::from("<<"),
                TokenKind::BinaryOperators,
                TokenType::BitwiseLeftShift,
                14,
                14,
            ),
            Token::new(
                String::from("=="),
                TokenKind::BinaryOperators,
                TokenType::Equal,
                15,
                15,
            ),
            Token::new(
                String::from("!="),
                TokenKind::BinaryOperators,
                TokenType::NotEqual,
                16,
                16,
            ),
            Token::new(
                String::from("&"),
                TokenKind::BinaryOperators,
                TokenType::BitwiseAnd,
                17,
                17,
            ),
            Token::new(
                String::from("&&"),
                TokenKind::BinaryOperators,
                TokenType::LogicalAnd,
                18,
                18,
            ),
            Token::new(
                String::from("|"),
                TokenKind::BinaryOperators,
                TokenType::BitwiseOr,
                19,
                19,
            ),
            Token::new(
                String::from("||"),
                TokenKind::BinaryOperators,
                TokenType::LogicalOr,
                20,
                20,
            ),
            Token::new(
                String::from("="),
                TokenKind::AssignmentOperators,
                TokenType::Assignment,
                21,
                21,
            ),
            Token::new(
                String::from("+="),
                TokenKind::AssignmentOperators,
                TokenType::PlusEqual,
                22,
                22,
            ),
            Token::new(
                String::from("-="),
                TokenKind::AssignmentOperators,
                TokenType::MinusEqual,
                23,
                23,
            ),
            Token::new(
                String::from("/="),
                TokenKind::AssignmentOperators,
                TokenType::DivideEqual,
                24,
                24,
            ),
            Token::new(
                String::from("%="),
                TokenKind::AssignmentOperators,
                TokenType::ModuleEqual,
                25,
                25,
            ),
            Token::new(
                String::from("->"),
                TokenKind::SpecialChars,
                TokenType::Arrow,
                26,
                26,
            ),
            Token::new(
                String::from("::"),
                TokenKind::SpecialChars,
                TokenType::ScopeResolution,
                27,
                27,
            ),
            Token::new(
                String::from("?"),
                TokenKind::BinaryOperators,
                TokenType::TernaryOpetator,
                28,
                28,
            ),
        ];
        debug_assert_eq!(tok_vector, tok.tokenize());
    }

    #[test]
    fn test_tokenize_punctuations() {
        let text = "{\n }\n (\n)\n [\n ]\n :\n ;\n ,\n";
        let tok = Tokenizer::new(&text);
        let tok_vector = vec![
            Token::new(
                String::from("{"),
                TokenKind::SpecialChars,
                TokenType::LeftCurlyBrace,
                0,
                0,
            ),
            Token::new(
                String::from("}"),
                TokenKind::SpecialChars,
                TokenType::RightCurlyBrace,
                1,
                1,
            ),
            Token::new(
                String::from("("),
                TokenKind::SpecialChars,
                TokenType::LeftBracket,
                2,
                2,
            ),
            Token::new(
                String::from(")"),
                TokenKind::SpecialChars,
                TokenType::RightBracket,
                3,
                3,
            ),
            Token::new(
                String::from("["),
                TokenKind::SpecialChars,
                TokenType::LeftSquareBracket,
                4,
                4,
            ),
            Token::new(
                String::from("]"),
                TokenKind::SpecialChars,
                TokenType::RightSquareBracket,
                5,
                5,
            ),
            Token::new(
                String::from(":"),
                TokenKind::SpecialChars,
                TokenType::Colon,
                6,
                6,
            ),
            Token::new(
                String::from(";"),
                TokenKind::SpecialChars,
                TokenType::Semicolon,
                7,
                7,
            ),
            Token::new(
                String::from(","),
                TokenKind::SpecialChars,
                TokenType::Comma,
                8,
                8,
            ),
        ];
        assert_eq!(tok_vector, tok.tokenize());
    }

    #[test]
    fn test_tokenize_values() {
        let text = "\"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ`1234567890-=[]\\;\',./~!@#$%^&*()_+{}|:\\\"<>?\\\"\'\"\n'a'\n\'\\\'\'\n\'\\\"\'\n'\\\\'\n 1234567890\n1234567890.0987654321\ntrue\nfalse\n";
        let tok = Tokenizer::new(&text);
        let tok_vector = vec![
            Token::new(String::from("\"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ`1234567890-=[]\\;\',./~!@#$%^&*()_+{}|:\\\"<>?\\\"\'\""),
                       TokenKind::Values,
                       TokenType::StringValue,
                       0,
                       0),
            Token::new(String::from("'a'"), TokenKind::Values, TokenType::CharValue, 1, 1),
            Token::new(String::from("\'\\\'\'"), TokenKind::Values, TokenType::CharValue, 2, 2),
            Token::new(String::from("\'\\\"\'"), TokenKind::Values, TokenType::CharValue, 3, 3),
            Token::new(String::from("\'\\\\\'"), TokenKind::Values, TokenType::CharValue, 4, 4),
            Token::new(String::from("1234567890"), TokenKind::Values, TokenType::NumberInteger, 5, 5),
            Token::new(String::from("1234567890.0987654321"),
                       TokenKind::Values,
                       TokenType::NumberFloat,
                       6,
                       6),
            Token::new(String::from("true"), TokenKind::Values, TokenType::True, 7, 7),
            Token::new(String::from("false"), TokenKind::Values, TokenType::False, 8, 8)];
        assert_eq!(tok_vector, tok.tokenize());
    }

    #[test]
    fn test_tokenize_ids() {
        let text = "_\n _1123abcd_deff04\n abcd_deff04_\n integer\n main\n";
        let tok = Tokenizer::new(&text);
        let tok_vector = vec![
            Token::new(
                String::from("_"),
                TokenKind::Identifiers,
                TokenType::Identifier,
                0,
                0,
            ),
            Token::new(
                String::from("_1123abcd_deff04"),
                TokenKind::Identifiers,
                TokenType::Identifier,
                1,
                1,
            ),
            Token::new(
                String::from("abcd_deff04_"),
                TokenKind::Identifiers,
                TokenType::Identifier,
                2,
                2,
            ),
            Token::new(
                String::from("integer"),
                TokenKind::Identifiers,
                TokenType::Identifier,
                3,
                3,
            ),
            Token::new(
                String::from("main"),
                TokenKind::Identifiers,
                TokenType::Main,
                4,
                4,
            ),
        ];
        assert_eq!(tok_vector, tok.tokenize());
    }
}
