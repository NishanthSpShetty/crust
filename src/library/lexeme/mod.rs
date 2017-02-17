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
//--->Put Types before this and make changes in parser while inserting into symbol table

//---> Base Types for tokens for recognizing different token categories
    BASE_DATATYPE,
    BASE_BINOP,
    BASE_UNOP,
    BASE_ASSIGNOP,
    BASE_NONE,


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

    PRIMITIVE_TYPEDEF, // typdef => Type

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

    KEYWORD_UNSIGNED, // u16, u32, u64
    KEYWORD_SIGNED, // 

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

    OTHER, // for testing
}


#[derive(Debug)]
#[derive(PartialEq, Eq)]
pub struct Token {
    value: String,
    // base type of token
    base_type: Type,
    typ: Type,
    ln: i32,
    id:i32
}

impl Clone for Token {
    fn clone(&self) -> Token {
        let v = self.value.clone();
        Token { value: v, ..*self }
    }
}

impl Token {
    pub fn new(token: String, base_type: Type, tok_type: Type, line_no: i32,id_:i32) -> Token {
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

    pub fn get_token_ln(&self) -> i32 {
        self.ln
    }

    pub fn get_token_id(&self) -> i32 {
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

    fn set_token_ln(&mut self, ln: i32) {
        self.ln = ln;
    }
    fn set_token_id(&mut self, id_: i32) {
        self.id = id_;
    }
}
