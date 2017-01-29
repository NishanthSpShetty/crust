pub enum Type{
	DATATYPE,
	RETTYPE,
	OP_EQU,
	OP_NEQ,
	OP_PLUS,
	OP_MINUS,
	OP_MUL,
	OP_DIV,
	OP_MOD,
	OP_PLUSEQU,
	OP_MINEQU,
	OP_MULEQU,
	OP_DIVEQU,
	OP_BITAND,
	OP_BITOR,
	OP_BITNEG,
	OP_BITLSHIFT,
	OP_BITRSHIFT,
	OP_LOGAND,
	OP_LOGOR,
	OP_LOGNOT,
	OP_INC,
	OP_DEC,
	OP_LE,
	OP_GE,
	OP_ADDROF,
	OP_ASSIGN,

	STRING,
	//for specific type info
	INT, //i32
	FLOAT, //f32
	CHAR, //char
	DOUBLE, //f64
	SHORT, //i16
	
	LEFT_CBRACE,
	RIGHT_CBRACE,
	LEFT_BRACKET,
	RIGHT_BRACKET,
	LEFT_SBRACKET,
	RIGHT_SBRACKET,
	SEMICOLON,
	IDENTIFIER,
	KEYWORD_CLASS,
	KEYWORD_FOR,
	KEYWORD_WHILE,
	KEYWORD_DO,
	KEYWORD_BREAK,
	KEYWORD_CONTINUE,
	KEYWORD_SWITCH,
	KEYWORD_IF,
	KEWORD_ELSE,
	KEYWORD_PUBLIC,
	KEYWORD_PRIVATE,
	KEYWORD_PROTECTED,
	KEYWORD_CASE,
	KEYWORD_STATIC,
	KEYWORD_CONST,
	KEYWORD_DEFAULT,
	KEYWORD_RETURN,
	
};


pub struct Tokens{
	value:String,
	typ:Type,
	ln:i32,
	};


impl Tokens{

	fn get_token_type(&self)->Type{
		self.typ
	}

	fn get_token_value(&self)->String{
		self.value	
	}

	fn get_token_ln(&self)->i32{
		self.ln
	}
	
	fn set_token_value(&mut self,val:&str){
		self.value = val.to_string();
	}

	fn set_token_type(&mut self,typ:Type){
		self.typ = typ;
	}

	fn set_token_ln(&mut self,ln:i32){
		self.ln = ln;
	}
	
};
