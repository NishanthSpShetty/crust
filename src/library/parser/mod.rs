use library::lexeme::Type::*;
use library::lexeme::Token;

#[derive(Debug)]
struct SymbolTable {
    typ: i32,
    id_name: String,
    is_assigned: bool,
    assigned_val: String,
}

impl Clone for SymbolTable {
    fn clone(&self) -> SymbolTable {
        let id = self.id_name.clone();
        let val = self.assigned_val.clone();
        SymbolTable {
            assigned_val: val,
            id_name: id,
            ..*self
        }
    }
}



pub fn parse_program(lexeme: Vec<Token>) -> Vec<String> {

    let mut stream: Vec<String> = Vec::new();
    let mut head: usize = 0;
    let mut lookahead: usize = 0;
    let mut temp_lexeme: Vec<Token> = Vec::new();
    while head < lexeme.len() {
        match lexeme[head].get_token_type() {
            PRIMITIVE_INT => {
                lookahead += 2;
                match lexeme[lookahead].get_token_type() {
                    LEFT_BRACKET => {
                        while lexeme[lookahead].get_token_type() != RIGHT_CBRACE {
                            lookahead += 1;
                        }
                        lookahead += 1;
                        while head != lookahead {
                            let l: Token = Token::new(lexeme[head].get_token_value(),
                                                      lexeme[head].get_token_type(),
                                                      lexeme[head].get_token_ln(),
                                                      lexeme[head].get_token_id());
                            temp_lexeme.push(l);
                            head += 1;
                        }
                        parse_function(&temp_lexeme);
                        temp_lexeme.clear();
                    }
                    SEMICOLON | COMMA | OP_ASSIGN => {
                        while lexeme[lookahead].get_token_type() != SEMICOLON {
                            lookahead += 1;
                        }

                        lookahead += 1;
                        while head != lookahead {
                            let l: Token = Token::new(lexeme[head].get_token_value(),
                                                      lexeme[head].get_token_type(),
                                                      lexeme[head].get_token_ln(),
                                                      lexeme[head].get_token_id());
                            temp_lexeme.push(l);
                            head += 1;
                        }
                        stream.append(&mut parse_declaration(&temp_lexeme));
                        temp_lexeme.clear();
                    }

                    _ => {}
                };
            }
            KEYWORD_IF => {
                println!("if found");
                while lexeme[lookahead].get_token_type() != SEMICOLON {
                    lookahead += 1;
                }
                lookahead += 1;

                while head != lookahead {
                    let l: Token = Token::new(lexeme[head].get_token_value(),
                                              lexeme[head].get_token_type(),
                                              lexeme[head].get_token_ln(),
                                              lexeme[head].get_token_id());
                    temp_lexeme.push(l);
                    head += 1;
                }
                stream.append(&mut parse_if(&temp_lexeme));
                temp_lexeme.clear();
            }
            _ => {}
        };
    }
    //return the rust lexeme to main
    stream
}


fn parse_function(lexeme: &Vec<Token>) {
    println!(" Unimplememted {:?}", lexeme);
}

fn parse_declaration(lexeme: &Vec<Token>) -> Vec<String> {
    //println!("\n Implementing declaration converter for {:?} ", lexeme);
    let mut sym_tab: Vec<SymbolTable> = Vec::new();
    let mut sym: SymbolTable = SymbolTable {
        typ: -1,
        id_name: "NONE".to_string(),
        is_assigned: false,
        assigned_val: "NONE".to_string(),
    };
    let mut head: usize = 1;
    //let sym_idx:usize=0;
    while head < lexeme.len() {

        match lexeme[head].get_token_type() {
            IDENTIFIER => sym.id_name = lexeme[head].get_token_value(),
            OP_ASSIGN => {
                head += 1;
                sym.assigned_val = lexeme[head].get_token_value();
                sym.is_assigned = true;
            }

            SEMICOLON | COMMA => {
                sym.typ = 0;
                sym_tab.push(sym.clone());
            }
            _ => {}
        };
        head += 1;

    }
    println!(" {:?}", sym_tab);

    let mut stream: Vec<String> = Vec::new();
    for i in &sym_tab {
        stream.push("let".to_string());
        stream.push(i.id_name.clone());
        stream.push(":".to_string());
        match i.typ {
            0 => stream.push("i32".to_string()),
            1 => stream.push("f32".to_string()),
            _ => stream.push("UNKNOWN_TYPE".to_string()),
        }
        stream.push(";".to_string());
    }

    stream
}

fn parse_if(lexeme: &Vec<Token>) -> Vec<String> {
    let mut i: usize = 0;
    let mut cond: String = String::new();
    let mut stream: Vec<String> = Vec::new();

    while i < lexeme.len() {
        match lexeme[i].get_token_type() {
            KEYWORD_IF => stream.push("if".to_string()),
            LEFT_BRACKET => {
                i += 1;
                while lexeme[i].get_token_type() != RIGHT_BRACKET {
                    cond = cond + &lexeme[i].get_token_value()[..];
                    i += 1;
                }
                stream.push(String::from(&cond[..]));

                stream.push("{\n".to_string());

            }
            LEFT_CBRACE => {}
            RIGHT_CBRACE => {}

            _ => stream.push(lexeme[i].get_token_value()),
        }
        i += 1;
    }

    stream.push("\n}".to_string());
    stream
}
