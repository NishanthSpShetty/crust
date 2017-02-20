#[allow(dead_code)]
#[derive(Debug)]
#[derive(Clone,Copy)]
#[derive(PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum Type {
    // for specific type info
    PRIMITIVE_INT, // i32 //
    PRIMITIVE_SHORT, // i16
    PRIMITIVE_LONG, // i64
    PRIMITIVE_FLOAT, // f32 //
    PRIMITIVE_DOUBLE, // f64
    PRIMITIVE_CHAR, // char //
    PRIMITIVE_BOOL, // bool
    PRIMITIVE_VOID,
    //---> Put Types before this and make changes in parser while inserting into symbol table

//---> Unimplemented Types
    PRIMITIVE_TYPEDEF, // typdef => Type
    KEYWORD_UNSIGNED, // u16, u32, u64
    KEYWORD_SIGNED, //

    //---> Put Base Types for tokens here for recognizing different token categories
    BASE_DATATYPE,
    BASE_BINOP,
    BASE_UNOP,
    BASE_ASSIGNOP,
    BASE_COMMENT,
    BASE_VALUE,
    BASE_NONE,

    //---> Put operators here
    OP_EQU, //
    OP_NEQ, //
    OP_PLUS, //
    OP_MINUS, //
    OP_MUL, //
    OP_DIV, //
    OP_MOD, //
    OP_PLUSEQU, //
    OP_MINEQU, //
    OP_MULEQU, //
    OP_DIVEQU, //
    OP_MODEQU, //
    OP_BITAND, //
    OP_BITOR, //
    OP_BITNEG, //
    OP_BITLSHIFT, //
    OP_BITRSHIFT, //
    OP_LOGAND, //
    OP_LOGOR, //
    OP_LOGNOT, //
    OP_INC, //
    OP_DEC, //
    OP_GT, //
    OP_LT, //
    OP_LE, //
    OP_GE, //
    OP_ADDROF, // differentiate b/w this and OP_MUL during parsing?
    OP_ASSIGN, //
    OP_INDIRECT, // differentiate b/w this and OP_BITAND during parsing?

    RETTYPE,

    //---> Values here
    CHAR_VAL, //
    STRING, //
    NUM_INT, //
    NUM_FLOAT, //
    TRUE_VAL, //
    FALSE_VAL, //

    //---> Punctuations here
    LEFT_CBRACE, //
    RIGHT_CBRACE, //
    LEFT_BRACKET, //
    RIGHT_BRACKET, //
    LEFT_SBRACKET, //
    RIGHT_SBRACKET, //
    COLON, //
    SEMICOLON, //
    COMMA, //

    //---> Comments here
    COMMENT_SINGLE, //
    COMMENT_MULTI, //

    IDENTIFIER, //

    //---> Keywords here
    MAIN,
    KEYWORD_CLASS, //
    KEYWORD_NEW,
    KEYWORD_FOR, //
    KEYWORD_WHILE, //
    KEYWORD_DO, //
    KEYWORD_BREAK, //
    KEYWORD_CONTINUE, //
    KEYWORD_SWITCH, //
    KEYWORD_IF, //
    KEYWORD_ELSE, //
    KEYWORD_PUBLIC, //
    KEYWORD_PRIVATE, //
    KEYWORD_PROTECTED, //
    KEYWORD_CASE, //
    KEYWORD_STATIC, //
    KEYWORD_CONST, //
    KEYWORD_DEFAULT, //
    KEYWORD_RETURN, //

    //---> If all fails
    OTHER,
}


#[derive(Debug)]
#[derive(PartialEq, Eq)]
pub struct Token {
    value: String,
    // base type of token
    base_type: Type,
    typ: Type,
    ln: u32,
    id: u32,
}

impl Clone for Token {
    fn clone(&self) -> Token {
        let v = self.value.clone();
        Token { value: v, ..*self }
    }
}

impl Token {
    pub fn new(token: String, base_type: Type, tok_type: Type, line_no: u32, id_: u32) -> Token {
        Token {
            value: token,
            base_type: base_type,
            typ: tok_type,
            ln: line_no,
            id: id_,
        }
    }

    // returns both base type and token type. Used in parse_program
    pub fn get_type(&self) -> (Type, Type) {
        (self.base_type, self.typ)
    }
    pub fn get_token_type(&self) -> Type {
        self.typ
    }

    pub fn get_base_type(&self) -> Type {
        self.base_type
    }

    pub fn get_token_value(&self) -> String {
        self.value.clone()
    }

    pub fn get_token_ln(&self) -> u32 {
        self.ln
    }

    pub fn get_token_id(&self) -> u32 {
        self.id
    }

    fn set_token_value(&mut self, val: &str) {
        self.value = val.to_string();
    }

    fn set_token_type(&mut self, typ: Type) {
        self.typ = typ;
    }

    fn set_base_type(&mut self, typ: Type) {
        self.base_type = typ;
    }

    fn set_token_ln(&mut self, ln: u32) {
        self.ln = ln;
    }
    fn set_token_id(&mut self, id_: u32) {
        self.id = id_;
    }
}

#[cfg(test)]
mod test {
    use library::lexeme::*;
    
    #[test]
    fn test_new_token() {
        let token: Token = Token::new("Hello World".to_string(), Type::BASE_NONE, Type::STRING, 0, 0);
        assert_eq!(token.value, "Hello World".to_string());
        assert_eq!(token.base_type, Type::BASE_NONE);
        assert_eq!(token.typ, Type::STRING);
        assert_eq!(token.ln, 0);
        assert_eq!(token.id, 0);
    }
    
    #[test]
    fn test_get_set_token_value() {
        let mut token: Token = Token::new('\''.to_string(), Type::BASE_NONE, Type::CHAR_VAL, 0, 0);
        assert_eq!(token.get_token_value(), "\'");

        token.set_token_value(&"\"");
        assert_eq!(token.get_token_value(), "\"");
    }
    #[test]
    fn test_get_set_token_type() {
        let mut token: Token = Token::new("12abcd".to_string(), Type::BASE_NONE, Type::OTHER, 0, 0);
        assert_eq!(token.get_token_type(), Type::OTHER);

        token.set_token_type(Type::PRIMITIVE_TYPEDEF);
        assert_eq!(token.get_token_type(), Type::PRIMITIVE_TYPEDEF);
    }

    #[test]
    fn test_get_set_base_type() {
        let mut token: Token = Token::new("12.03".to_string(), Type::BASE_NONE, Type::NUM_FLOAT, 0, 0);
        assert_eq!(token.get_base_type(), Type::BASE_NONE);

        token.set_base_type(Type::BASE_VALUE);
        assert_eq!(token.get_base_type(), Type::BASE_VALUE);
    }

    #[test]
    fn test_get_set_line_no() {
        let mut token: Token = Token::new("12.03".to_string(), Type::BASE_NONE, Type::NUM_FLOAT, 0, 0);
        assert_eq!(token.get_token_ln(), 0);

        token.set_token_ln(6);
        assert_eq!(token.get_token_ln(), 6);
    }

    #[test]
    fn test_get_set_id() {
        let mut token: Token = Token::new("12.03".to_string(), Type::BASE_NONE, Type::NUM_FLOAT, 0, 0);
        assert_eq!(token.get_token_id(), 0);

        token.set_token_id(4);
        assert_eq!(token.get_token_id(), 4);
    }
}