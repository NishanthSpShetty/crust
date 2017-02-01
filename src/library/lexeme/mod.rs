#[allow(dead_code)]
#[derive(Debug)]
#[derive(Clone,Copy)]
#[derive(PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum Type {
    DATATYPE,
    RETTYPE,
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

    STRING, //
    // for specific type info
    PRIMITIVE_INT, // i32 //
    PRIMITIVE_FLOAT, // f32 //
    PRIMITIVE_CHAR, // char //
    PRIMITIVE_DOUBLE, // f64
    PRIMITIVE_SHORT, // i16
    PRIMITIVE_LONG, //
    PRIMITIVE_TYPEDEF, // typdef => Type
    PRIMITIVE_BOOL, //

    CHAR_VAL, //
    NUM_INT, //
    NUM_FLOAT, //
    TRUE_VAL, //
    FALSE_VAL, //

    LEFT_CBRACE, //
    RIGHT_CBRACE, //
    LEFT_BRACKET, //
    RIGHT_BRACKET, //
    LEFT_SBRACKET, //
    RIGHT_SBRACKET, //
    COLON, //
    SEMICOLON, //
    COMMA, //
    COMMENT_SINGLE, //
    COMMENT_MULTI, //

    IDENTIFIER, //

    KEYWORD_UNSIGNED, //
    KEYWORD_SIGNED, //

    KEYWORD_CLASS, //
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

    OTHER, // for testing
}

// trait Clone {
// 	 fn clone(&self) -> Type
// }


// impl Clone for Type {
//     fn clone(&self) -> Type { *self }
// }

// impl fmt::Debug for Type {
// 	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
// 		write!(f, "{}", );
// 	}
// }

#[derive(Debug)]
#[derive(PartialEq, Eq)]
pub struct Token {
    value: String,
    typ: Type,
    ln: i32,
}

impl Clone for Token {
    fn clone(&self) -> Token {
        let v = self.value.clone();
        Token { value: v, ..*self }
    }
}

impl Token {
    pub fn new(token: String, tok_type: Type, line_no: i32) -> Token {
        Token {
            value: token,
            typ: tok_type,
            ln: line_no,
        }
    }

    pub fn get_token_type(&self) -> Type {
        self.typ
    }

    pub fn get_token_value(&self) -> String {
        self.value.clone()
    }

    pub fn get_token_ln(&self) -> i32 {
        self.ln
    }

    fn set_token_value(&mut self, val: &str) {
        self.value = val.to_string();
    }

    fn set_token_type(&mut self, typ: Type) {
        self.typ = typ;
    }

    fn set_token_ln(&mut self, ln: i32) {
        self.ln = ln;
    }
}
