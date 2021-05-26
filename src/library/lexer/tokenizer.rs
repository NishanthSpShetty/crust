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
    pub fn new(text: &str) -> Tokenizer {
        let token: Vec<char> = Vec::new();
        let token_stream: Vec<Token> = Vec::new();

        Tokenizer {
            position: 0,
            id: 0,
            line_no: 0,
            current_char: BLACK_HOLE,
            length: text.len(),
            token: token,
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
