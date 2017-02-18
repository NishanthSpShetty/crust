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

static mut IN_BLOCK_STMNT:bool = false;

pub fn parse_program(lexeme:Vec<Token>) -> Vec<String> {

    let mut stream: Vec<String> = Vec::new();
    let mut head: usize = 0;
    let mut lookahead: usize = 0;
    let mut temp_lexeme: Vec<Token> = Vec::new();

    while head < lexeme.len() {
        // gets both base type and token type
        match lexeme[head].get_type() {

            // matches any datatype
            (BASE_DATATYPE, _) => {
                lookahead += 2;
                
                match lexeme[lookahead].get_token_type() {
                    
                    // function
                    LEFT_BRACKET => {
                        //inside the function
                       unsafe{ IN_BLOCK_STMNT = true;}
                        while lexeme[lookahead].get_token_type() != LEFT_CBRACE {
                            lookahead += 1;
                        }
                        lookahead = skip_block(&lexeme, lookahead+1);
                        while head < lookahead {
                            let l: Token = lexeme[head].clone();
                            temp_lexeme.push(l);
                            head += 1;
                        }
                        stream.append(&mut parse_function(&temp_lexeme));
                       unsafe{ IN_BLOCK_STMNT = false; }
                        temp_lexeme.clear();
                    }

                    // declaration or assignment
                    SEMICOLON | COMMA | OP_ASSIGN => {
                        
                        while lexeme[lookahead].get_token_type() != SEMICOLON {
                            lookahead += 1;
                        }
                        lookahead += 1;
                        while head != lookahead {
                            let l: Token = lexeme[head].clone();
                            temp_lexeme.push(l);
                            head += 1;
                        }
                        stream.append(&mut parse_declaration(&temp_lexeme));
                        temp_lexeme.clear();
                    }

                    _ => {}
                };
            }
            (_,KEYWORD_IF) => {
                let mut temp_lexeme: Vec<Token> = Vec::new();
                while lexeme[lookahead].get_token_type() != RIGHT_BRACKET {
                    lookahead += 1;
                }
                lookahead += 1;
                if lexeme[lookahead].get_token_type() == LEFT_CBRACE {
                   unsafe {IN_BLOCK_STMNT = true;}
                    lookahead = skip_block(&lexeme, lookahead+1);
                }
                else {
                    lookahead = skip_stmt(&lexeme, lookahead);
                }
                
                while head < lookahead {
                    let l: Token = lexeme[head].clone();
                    temp_lexeme.push(l);
                    head += 1;
                }
                stream.append(&mut parse_if(&temp_lexeme));

                // add if without braces
            }
            
            (BASE_COMMENT, _) => {
                stream.push(lexeme[head].get_token_value()+"\n;");
                head+=1;
            },
            (_,_) => {
                if lexeme[head].get_token_type() != RIGHT_CBRACE{
                stream.push(lexeme[head].get_token_value());
                }
                head+=1;
            },

        };
    
    }
    //return the rust lexeme to main
    
    stream
}

// for debugging
fn print_lexemes(lexeme: &Vec<Token>, start: usize, end: usize) {
    println!("----------lexeme-start------------");
    for i in start..end {
        println!("{}>>>{}", i, lexeme[i].get_token_value());
    }
    println!("----------lexeme-end------------");
}
fn skip_stmt(lexeme: &Vec<Token>, mut lookahead: usize)->usize {
    while lexeme[lookahead].get_token_type() != SEMICOLON {
        lookahead += 1;
    }
    lookahead+1
}
fn skip_block(lexeme: &Vec<Token>, mut lookahead: usize)->usize {
    let mut paren = 1;
    while paren != 0 && lookahead < lexeme.len() { // is the second condition really required?
        if lexeme[lookahead].get_token_type() == LEFT_CBRACE {
            paren += 1;
        }
        if lexeme[lookahead].get_token_type() == RIGHT_CBRACE {
            paren -= 1;
        }
        lookahead+=1;
    }
    lookahead
}


/**
 * parse_function
 * this function will parse c/c++ function into rust
 * equivalent function definition
 */ 


fn parse_function(lexeme: &Vec<Token>)->Vec<String> {
    let mut temp_lexeme:Vec<Token> = Vec::new();
    let mut head:usize=3;
    
    let mut stream:Vec<String> = Vec::new();
    stream.push("fn".to_string());
    stream.push(lexeme[1].get_token_value());
    stream.push("(".to_string());
    if lexeme[1].get_token_type()!=MAIN{
       
    //parse the argument
    while lexeme[head].get_token_type() != RIGHT_BRACKET {
        
        // push identifier
        stream.push(lexeme[head+1].get_token_value()); //int f(int val)
        stream.push(":".to_string());
        
        // parse argument type
        if let Some(rust_type) = parse_type(lexeme[head].get_token_type() as i32) {
            stream.push(rust_type);
        }
        
        head+=2
    }
    stream.push(")".to_string());
    stream.push("->".to_string());
    
    // parse return type
    if let Some(rust_type) = parse_type(lexeme[0].get_token_type() as i32) {
        stream.push(rust_type);
    }
    }else {
        stream.push(")".to_string());
    }
    stream.push("{".to_string());
    
    //parse the function body
    while lexeme[head].get_token_type() != LEFT_CBRACE { head+=1 }
    head+=1;
    while head < lexeme.len()-1 {
        let l: Token = lexeme[head].clone();
        temp_lexeme.push(l);
        head += 1;
    }
    stream.append(&mut parse_program(temp_lexeme));
    stream.push("}".to_string());
    stream
}

/**
 * parse_declaration
 * this function will parse c/c++ declaration into rust
 * equivalent statements
 */ 

fn parse_declaration(lexeme: &Vec<Token>) -> Vec<String> {
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
                // used enum value in the symbol table
                sym.typ = lexeme[0].get_token_type() as i32;
                sym_tab.push(sym.clone());
            }
            _ => {}
        };
        head += 1;

    }

    let mut stream: Vec<String> = Vec::new();
    for i in &sym_tab {

        // get identifier
        //for declaration out of any blocks(global)
       unsafe{
        if IN_BLOCK_STMNT == false { stream.push("static".to_string());}
        else { stream.push("let".to_string()); }
       }
       stream.push(i.id_name.clone());
        stream.push(":".to_string());

        // get the rust type
        if let Some(rust_type) = parse_type(i.typ) {
            stream.push(rust_type);
        }
        else {
            stream.push("UNKNOWN_TYPE".to_string());
        }

        // take care of assignment
        if i.is_assigned {
            stream.push("=".to_string());
            stream.push((&i.assigned_val).to_string());
        }
        stream.push(";".to_string());
    }
    stream
}



/**
 * parse_if
 * this function will parse c/c++ if statements into rust
 * equivalent statements
 */ 

fn parse_if(lexeme: &Vec<Token>) -> Vec<String> {
    let mut stream: Vec<String> = Vec::new();
    let mut head: usize = 0;
    
    stream.push("if".to_string());
    head += 1;

    //skip '('
    head += 1;
    
    // condition
    while lexeme[head].get_token_type() != RIGHT_BRACKET {
                    stream.push(lexeme[head].get_token_value());
                    head+=1;
    }
    head += 1;
    //stream.push(")".to_string());
    stream.push("{".to_string());

    // change end to right brace if braces are used
    if lexeme[head].get_token_type() == LEFT_CBRACE {
        // end = RIGHT_CBRACE;
        head += 1;
        // if block
    }
    // head += 1;

    let mut temp_lexeme: Vec<Token> = Vec::new();
    while head < lexeme.len() {
        let l: Token = lexeme[head].clone();
        temp_lexeme.push(l);
        head += 1;
    }
    stream.append(&mut parse_program(temp_lexeme));
    stream.push("}".to_string());
    stream
}


/**
* fn parse_type
*   c_type : integer value of Type
*   return : either the equivalent rust type as a string or None, if does not correspond to any c datatype
*/ 
fn parse_type(c_type: i32) -> Option<String> {
    match c_type {
        0 => Some("i32".to_string()),
        1 => Some("i16".to_string()),
        2 => Some("i64".to_string()),
        3 => Some("f32".to_string()),
        4 => Some("f64".to_string()),
        5 => Some("char".to_string()),
        6 => Some("bool".to_string()),
        _ => {None}
    }
}
