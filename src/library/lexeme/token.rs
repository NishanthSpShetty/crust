use crate::library::lexeme::definition::{TokenKind, TokenType};
use std::fmt;

#[derive(Debug, PartialEq, Eq)]
pub struct Token {
    token: String,
    token_kind: TokenKind,
    token_type: TokenType,
    line_number: u32,
    id: u32,
}

impl Clone for Token {
    fn clone(&self) -> Token {
        let v = self.token.clone();
        Token { token: v, ..*self }
    }
}

impl Token {
    pub fn new(
        token: String,
        token_kind: TokenKind,
        token_type: TokenType,
        line_number: u32,
        id: u32,
    ) -> Token {
        Token {
            token,
            token_kind,
            token_type,
            line_number,
            id,
        }
    }

    // returns both token kind and token type.
    pub fn get_type(&self) -> (TokenKind, TokenType) {
        (self.token_kind, self.token_type)
    }
    pub fn get_token_type(&self) -> TokenType {
        self.token_type
    }

    pub fn get_token_kind(&self) -> TokenKind {
        self.token_kind
    }

    pub fn get_token_value(&self) -> String {
        self.token.clone()
    }

    pub fn get_token_line_num(&self) -> u32 {
        self.line_number
    }

    pub fn get_token_id(&self) -> u32 {
        self.id
    }

    pub fn set_token_value(&mut self, val: &str) {
        self.token = val.to_string();
    }

    fn set_token_type(&mut self, typ: TokenType) {
        self.token_type = typ;
    }

    fn set_base_type(&mut self, typ: TokenKind) {
        self.token_kind = typ;
    }

    fn set_token_ln(&mut self, ln: u32) {
        self.line_number = ln;
    }
    fn set_token_id(&mut self, id_: u32) {
        self.id = id_;
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "[Token ID {}, Token: {}, Kind : {:?}, Type: {:?}, LineNumber : {}]",
            self.id, self.token, self.token_kind, self.token_type, self.line_number,
        )
    }
}

#[cfg(test)]
mod test {
    use crate::library::lexeme::{definition::*, token::Token};

    #[test]
    fn test_that_token_created_with_new_successfully() {
        let token: Token = Token::new(
            "Hello World".to_string(),
            TokenKind::Values,
            TokenType::StringValue,
            0,
            0,
        );
        assert_eq!(token.token, "Hello World".to_string());
        assert_eq!(token.token_kind, TokenKind::Values);
        assert_eq!(token.token_type, TokenType::StringValue);
        assert_eq!(token.line_number, 0);
        assert_eq!(token.id, 0);
    }

    #[test]
    fn test_that_token_value_can_set_and_read_successfully() {
        let mut token: Token = Token::new(
            '\''.to_string(),
            TokenKind::Values,
            TokenType::CharValue,
            0,
            0,
        );
        assert_eq!(token.get_token_value(), "\'");

        token.set_token_value(&"\"");
        assert_eq!(token.get_token_value(), "\"");
    }

    #[test]
    fn test_that_token_type_can_set_and_read_successfully() {
        let mut token: Token = Token::new(
            "int".to_string(),
            TokenKind::DataTypes,
            TokenType::Integer,
            0,
            0,
        );
        assert_eq!(token.get_token_type(), TokenType::Integer);

        token.set_token_type(TokenType::Typedef);
        assert_eq!(token.get_token_type(), TokenType::Typedef);
    }

    #[test]
    fn test_that_token_kind_can_set_and_read_successfully() {
        let mut token: Token = Token::new(
            "12.03".to_string(),
            TokenKind::Values,
            TokenType::NumberFloat,
            0,
            0,
        );
        assert_eq!(token.get_token_kind(), TokenKind::Values);

        token.set_base_type(TokenKind::Typedef);
        assert_eq!(token.get_token_kind(), TokenKind::Typedef);
    }

    #[test]
    fn test_that_token_line_number_can_set_and_read_successfully() {
        let mut token: Token = Token::new(
            "12.03".to_string(),
            TokenKind::Values,
            TokenType::NumberFloat,
            0,
            0,
        );
        assert_eq!(token.get_token_line_num(), 0);

        token.set_token_ln(6);
        assert_eq!(token.get_token_line_num(), 6);
    }

    #[test]
    fn test_that_token_id_can_set_and_read_successfully() {
        let mut token: Token = Token::new(
            "12.03".to_string(),
            TokenKind::Values,
            TokenType::NumberFloat,
            0,
            0,
        );
        assert_eq!(token.get_token_id(), 0);

        token.set_token_id(4);
        assert_eq!(token.get_token_id(), 4);
    }
}
