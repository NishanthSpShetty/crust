use library::doc::DocType::*;
use library::lexeme::Type::*;
use library::lexeme::Token;

#[derive(Debug)]
struct SymbolTable {
    typ: i32,
    id_name: String,
    is_assigned: bool,
    is_ptr: bool,
    assigned_val: String,
}
#[derive(Debug)]
struct StructMem {
    name: String,
    typ: i32,
    identifier: String,
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
static mut ONCE_WARNED: bool = false;
static mut IN_BLOCK_STMNT: bool = false;
static mut IN_EXPR: bool = false;
static mut IN_SWITCH: bool = false;
static mut strict: bool = true;



pub fn init_parser(lexeme: &Vec<Token>, strict_parser: bool) -> Vec<String> {
    unsafe {
        strict = strict_parser;
    }
    let mut stream: Vec<String> = Vec::new();
    stream.push(CRUST.get_doc().to_string());
    stream.append(&mut parse_program(&lexeme));
    stream
}
/**
 * parse_program:
 * parse c program from Token Vector
 * return String Vector of Rust equivalent code
 */
fn parse_program(lexeme: &Vec<Token>) -> Vec<String> {
    let mut struct_mem: Vec<StructMem> = Vec::new();
    let mut stream: Vec<String> = Vec::new();
    let mut head: usize = 0;
    let mut lookahead: usize;
    let mut temp_lexeme: Vec<Token> = Vec::new();
    while head < lexeme.len() {
        // gets both base type and token type
        lookahead = head;
        match lexeme[head].get_type() {

            // matches any datatype
            (BASE_DATATYPE, _) => {
                lookahead += 2;
                match lexeme[lookahead].get_token_type() {

                    // function
                    LEFT_BRACKET => {
                        //inside the function
                        unsafe {
                            IN_BLOCK_STMNT = true;
                        }
                        while lexeme[lookahead].get_token_type() != RIGHT_BRACKET {
                            lookahead += 1;
                        }
                        lookahead += 1;

                        // skip function declaration
                        if lexeme[lookahead].get_token_type() != LEFT_CBRACE {
                            lookahead += 1;
                            head = lookahead;
                            continue;
                        }
                        // advance lookahead to end of block
                        lookahead = skip_block(&lexeme, lookahead + 1);
                        // collect entire function block
                        while head < lookahead {
                            let l: Token = lexeme[head].clone();
                            temp_lexeme.push(l);
                            head += 1;
                        }

                        //parse function block
                        stream.append(&mut parse_function(&temp_lexeme));
                        temp_lexeme.clear();

                        unsafe {
                            IN_BLOCK_STMNT = false;
                        }
                    }

                    //array declaration found
                    LEFT_SBRACKET => {
                        lookahead = skip_stmt(&lexeme, lookahead);

                        // collect variable declaration
                        while head != lookahead {
                            let l: Token = lexeme[head].clone();
                            temp_lexeme.push(l);
                            head += 1;
                        }
                        // parse declaration
                        stream.append(&mut parse_array_declaration(&temp_lexeme));
                        temp_lexeme.clear();
                    }

                    // variable declaration or declaration + assignment
                    SEMICOLON | COMMA | OP_ASSIGN => {
                        lookahead = skip_stmt(&lexeme, lookahead);

                        // collect variable declaration
                        while head != lookahead {
                            let l: Token = lexeme[head].clone();
                            temp_lexeme.push(l);
                            head += 1;
                        }
                        // parse declaration
                        stream.append(&mut parse_declaration(&temp_lexeme));
                        temp_lexeme.clear();
                    }
                    IDENTIFIER => {
                        //in case of pointer declaration
                        while lexeme[head].get_token_type() != SEMICOLON {
                            temp_lexeme.push(lexeme[head].clone());
                            head += 1;
                        }
                        temp_lexeme.push(lexeme[head].clone());
                        stream.append(&mut parse_declaration(&temp_lexeme));
                        head += 1;
                        println!("{}", lexeme[head].get_token_value());
                        temp_lexeme.clear();


                    }
                    _ => {}
                };
            }

            // matches if statement
            (_, KEYWORD_IF) => {
                // let mut temp_lexeme: Vec<Token> = Vec::new();

                // move lookahead past conditon
                while lexeme[lookahead].get_token_type() != RIGHT_BRACKET {
                    lookahead += 1;
                }
                lookahead += 1;

                // move lookahead past block
                if lexeme[lookahead].get_token_type() == LEFT_CBRACE {
                    lookahead = skip_block(&lexeme, lookahead + 1);
                }
                // move lookahead past block for 'if' without braces
                else {
                    lookahead = skip_stmt(&lexeme, lookahead);
                }
                // collect if block
                while head < lookahead {
                    let l: Token = lexeme[head].clone();
                    temp_lexeme.push(l);
                    head += 1;
                }

                // parse if
                stream.append(&mut parse_if(&temp_lexeme));
                temp_lexeme.clear();
            }

            (_, KEYWORD_ELSE) => {
                stream.push("else".to_string());
                head += 1;
                lookahead = head;
                if lexeme[head].get_token_type() == KEYWORD_IF {
                    continue;
                } else {
                    if lexeme[lookahead].get_token_type() == LEFT_CBRACE {
                        head += 1;
                        lookahead = skip_block(&lexeme, head) - 1;
                    } else {
                        lookahead = skip_stmt(&lexeme, lookahead);
                    }

                    while head < lookahead {
                        let l: Token = lexeme[head].clone();
                        temp_lexeme.push(l);
                        head += 1;
                    }
                    //** parse else body
                    stream.push("{".to_string());
                    stream.append(&mut parse_program(&temp_lexeme));
                    temp_lexeme.clear();
                    stream.push("}".to_string());
                }
            }

            (_, KEYWORD_SWITCH) => {

                while lexeme[lookahead].get_token_type() != LEFT_CBRACE {
                    lookahead += 1;
                }
                lookahead += 1;

                lookahead = skip_block(&lexeme, lookahead);
                while head < lookahead {
                    let l: Token = lexeme[head].clone();
                    temp_lexeme.push(l);
                    head += 1;
                }
                unsafe {
                    IN_SWITCH = true;
                }
                stream.append(&mut parse_switch(&temp_lexeme));
                temp_lexeme.clear();
                unsafe {
                    IN_SWITCH = false;
                }
            }

            (_, KEYWORD_WHILE) => {
                // let mut temp_lexeme: Vec<Token> = Vec::new();

                // move lookahead past conditon
                while lexeme[lookahead].get_token_type() != RIGHT_BRACKET {
                    lookahead += 1;
                }
                lookahead += 1;

                // move lookahead past block
                if lexeme[lookahead].get_token_type() == LEFT_CBRACE {
                    lookahead = skip_block(&lexeme, lookahead + 1);
                }
                // move lookahead past block for 'if' without braces
                else {
                    lookahead = skip_stmt(&lexeme, lookahead);
                }
                // collect if block
                while head < lookahead {
                    let l: Token = lexeme[head].clone();
                    temp_lexeme.push(l);
                    head += 1;
                }

                let was_in_switch: bool;
                unsafe {
                    was_in_switch = IN_SWITCH;
                    IN_SWITCH = false;
                }
                // parse if
                stream.append(&mut parse_while(&temp_lexeme));
                unsafe {
                    IN_SWITCH = was_in_switch;
                }
                temp_lexeme.clear();
            }

            // matches do while statement
            (_, KEYWORD_DO) => {

                // move lookahead past block
                lookahead = skip_block(&lexeme, lookahead + 2);
                lookahead = skip_stmt(&lexeme, lookahead);

                // collect while block
                while head < lookahead {
                    let l: Token = lexeme[head].clone();
                    temp_lexeme.push(l);
                    head += 1;
                }
                // parse while
                let was_in_switch: bool;
                unsafe {
                    was_in_switch = IN_SWITCH;
                    IN_SWITCH = false;
                }
                stream.append(&mut parse_dowhile(&temp_lexeme));
                temp_lexeme.clear();
                unsafe {
                    IN_SWITCH = was_in_switch;
                }
            }

            // matches for statement
            (_, KEYWORD_FOR) => {
                while lexeme[lookahead].get_token_type() != LEFT_CBRACE {
                    lookahead += 1;
                }
                lookahead += 1;
                lookahead = skip_block(&lexeme, lookahead);

                while head < lookahead {
                    let l: Token = lexeme[head].clone();
                    temp_lexeme.push(l);
                    head += 1;
                }

                let was_in_switch: bool;
                unsafe {
                    was_in_switch = IN_SWITCH;
                    IN_SWITCH = false;
                }
                stream.append(&mut parse_for(&temp_lexeme));
                temp_lexeme.clear();
                unsafe {
                    IN_SWITCH = was_in_switch;
                }
            }

            // matches single and multi-line comment
            (BASE_COMMENT, _) => {
                stream.push(lexeme[head].get_token_value() + "\n");
                head += 1;
            }

            // assignment statements
            (_, IDENTIFIER) => {
                // let mut temp_lexeme: Vec<Token> = Vec::new();
                //identifier = expr
                //identifier()
                //identifier+expr
                //identifier OP_INC|OP_DEC; =>postfix


                match lexeme[head + 1].get_type() {
                    (_, IDENTIFIER) => {
                        while lexeme[head].get_token_type() != SEMICOLON {
                            temp_lexeme.push(lexeme[head].clone());
                            head += 1;
                        }
                        temp_lexeme.push(lexeme[head].clone());
                        head += 1;
                        stream.append(&mut parse_class_decl(&temp_lexeme, &struct_mem));
                        temp_lexeme.clear();
                    }

                    (_, OP_ASSIGN) => {
                        // move lookahead past statement
                        if lexeme[head + 3].get_token_type() == COMMA {
                            lookahead = head + 3;
                            while head < lookahead + 1 {
                                let l: Token = lexeme[head].clone();
                                temp_lexeme.push(l);
                                head += 1;
                            }
                            stream.append(&mut parse_assignment(&temp_lexeme));
                            temp_lexeme.clear();
                        } else {
                            lookahead = skip_stmt(&lexeme, lookahead);
                            // collect statement
                            while head < lookahead {
                                let l: Token = lexeme[head].clone();
                                temp_lexeme.push(l);
                                head += 1;
                            }

                            // parse assignment
                            stream.append(&mut parse_assignment(&temp_lexeme));
                            temp_lexeme.clear();
                        }
                    }
                    (BASE_UNOP, _) => unsafe {
                        if IN_EXPR != true {
                            stream.push(lexeme[head].get_token_value());
                            stream.push(match lexeme[head + 1].get_token_type() {
                                OP_INC => "+=1".to_string(),
                                OP_DEC => "-=1".to_string(),
                                _ => " ;".to_string(),
                            });
                            head += 2;
                        } else {
                            head += 2;
                        }
                    },
                    (BASE_BINOP, _) => {
                        lookahead = skip_stmt(&lexeme, lookahead);

                        //check if overloaded operators is in effect like << >>
                        if lexeme[head + 2].get_token_type() == STRING ||
                           lexeme[head + 2].get_token_type() == CHAR_VAL {
                            stream.push("\n//This statement need to be handled manually \n"
                                .to_string());
                            while head < lookahead {
                                stream.push(lexeme[head].get_token_value());
                                head += 1;
                            }
                        } else {

                            // move lookahead past statement
                            // collect statement
                            while head < lookahead {
                                let l: Token = lexeme[head].clone();
                                temp_lexeme.push(l);
                                head += 1;
                            }

                            // parse assignment
                            stream.append(&mut parse_expr(&temp_lexeme));
                            temp_lexeme.clear();

                        }
                    }
                    (_, LEFT_BRACKET) => {
                        while lexeme[head].get_token_type() != RIGHT_BRACKET {
                            stream.push(lexeme[head].get_token_value());
                            head += 1;
                        }
                        stream.push(lexeme[head].get_token_value());
                        head += 1;
                    }
                    // (_, LEFT_SBRACKET) => {
                    //     while lexeme[head].get_token_type() != RIGHT_SBRACKET {
                    //         stream.push(lexeme[head].get_token_value());
                    //         head += 1;
                    //     }
                    //     stream.push(lexeme[head].get)
                    // }
                    (_, _) => {
                        if lexeme[head].get_token_type() != RIGHT_CBRACE {
                            stream.push(lexeme[head].get_token_value());
                        }
                        head += 1;
                    }
                };
            }

            (BASE_UNOP, _) => {
                stream.push(lexeme[head + 1].get_token_value());
                stream.push(match lexeme[head].get_token_type() {
                    OP_INC => "+=1".to_string(),
                    OP_DEC => "-=1".to_string(),
                    _ => " ;".to_string(),
                });
                head += 2;
            }

            (_, KEYWORD_STRUCT) => {
                if lexeme[head + 2].get_token_type() == LEFT_CBRACE {
                    //struct A{};
                    while lexeme[head].get_token_type() != RIGHT_CBRACE {
                        temp_lexeme.push(lexeme[head].clone());
                        head += 1;
                    }
                    //push the right curly brace
                    temp_lexeme.push(lexeme[head].clone());
                    stream.append(&mut parse_struct(&temp_lexeme, &mut struct_mem));
                    temp_lexeme.clear();
                    head += 2; //skip semicolon
                } else {
                    //struct variable declaration

                    while lexeme[head].get_token_type() != SEMICOLON {
                        temp_lexeme.push(lexeme[head].clone());
                        head += 1;
                    }
                    temp_lexeme.push(lexeme[head].clone());
                    head += 1;
                    stream.append(&mut parse_struct_decl(&temp_lexeme, &struct_mem));
                    temp_lexeme.clear();
                }
            }

            (_, KEYWORD_CLASS) => {

                if lexeme[head + 2].get_token_type() == LEFT_CBRACE {
                    //struct A{};
                    while lexeme[head].get_token_type() != RIGHT_CBRACE ||
                          lexeme[head + 1].get_token_type() != SEMICOLON {
                        temp_lexeme.push(lexeme[head].clone());
                        head += 1;
                    }

                    //push the right curly brace
                    temp_lexeme.push(lexeme[head].clone());
                    stream.append(&mut parse_class(&temp_lexeme, &mut struct_mem));
                    temp_lexeme.clear();
                    head += 2; //skip semicolon
                }
            }

            (_, KEYWORD_ENUM) => {
                while lexeme[head].get_token_type() != SEMICOLON {
                    stream.push(lexeme[head].get_token_value());
                    head += 1;
                }
                head += 1;
            }
            (_, KEYWORD_RETURN) => {
                let mut t = head;
                stream.push(NO_RETURN.get_doc().to_string());

                while lexeme[t].get_token_type() != SEMICOLON {
                    t += 1;
                }

                if t != lexeme.len() - 1 {
                    while lexeme[head].get_token_type() != SEMICOLON {
                        println!("{:?}", lexeme[head]);

                        stream.push(lexeme[head].get_token_value());
                        head += 1;
                    }
                    stream.push(lexeme[head].get_token_value());
                    head += 1;
                } else {
                    head += 1;

                    while lexeme[head].get_token_type() != SEMICOLON {
                        stream.push(lexeme[head].get_token_value());
                        head += 1;
                    }
                    head += 1;
                }
            }
            (_, INCLUDE) => {
                unsafe {
                    if ONCE_WARNED == false {
                        stream.push(INCLUDE_STMT.get_doc().to_string());
                    } else {
                        stream.pop();
                        stream.push("* >>>>>>>>".to_string());
                    }
                }
                while lexeme[head].get_token_type() != OP_GT {
                    stream.push(lexeme[head].get_token_value());
                    head += 1;
                }
                stream.push(lexeme[head].get_token_value() + "\n");
                stream.push("**/\n".to_string());
                head += 1;
                unsafe {
                    ONCE_WARNED = true;
                }

            }
            // if all fails
            (_, _) => {
                if lexeme[head].get_token_type() != RIGHT_CBRACE {
                    if lexeme[head].get_token_type() == COMMA {
                        stream.push(";".to_string());
                    } else if lexeme[head].get_token_type() == KEYWORD_BREAK {
                        unsafe {
                            if !IN_SWITCH {
                                stream.push(lexeme[head].get_token_value());
                            }
                        }
                    } else {
                        stream.push(lexeme[head].get_token_value());
                    }
                }
                head += 1;
            }

        };
    }
    //return the rust lexeme to main
    stream
}


/**
 * print_lexemes:
 * prints the lexemes in the lexeme vector
 * from index start to end
 */
fn print_lexemes(lexeme: &Vec<Token>, start: usize, end: usize) {
    println!("----------lexeme-start------------");
    for i in start..end {
        println!("{}> {}", i, lexeme[i].get_token_value());
    }
    println!("----------lexeme-end------------");
}


/**
 * skip_stmt:
 * forwards the lookahead by one statement
 * returns the lookahead at the lexeme after the semi-colon
 */
fn skip_stmt(lexeme: &Vec<Token>, mut lookahead: usize) -> usize {
    while lexeme[lookahead].get_token_type() != SEMICOLON {
        lookahead += 1;
    }
    lookahead + 1
}


/**
 * skip_block:
 * forwards the lookahead by one block
 * returns the lookahead at the lexeme after the closing brace
 */
fn skip_block(lexeme: &Vec<Token>, mut lookahead: usize) -> usize {
    let mut paren = 1;

    // while all braces are not closed
    // skip nested blocks if any
    while paren != 0 && lookahead < lexeme.len() {
        if lexeme[lookahead].get_token_type() == LEFT_CBRACE {
            paren += 1;
        }
        if lexeme[lookahead].get_token_type() == RIGHT_CBRACE {
            paren -= 1;
        }
        lookahead += 1;
    }
    lookahead
}


/**
 * parse_function:
 * parse c/c++ function into rust equivalent function
 */
fn parse_function(lexeme: &Vec<Token>) -> Vec<String> {
    let mut temp_lexeme: Vec<Token> = Vec::new();
    let mut head: usize = 3;
    let mut lookahead: usize = head;
    let mut stream: Vec<String> = Vec::new();

    stream.push("fn".to_string());
    stream.push(lexeme[1].get_token_value());
    stream.push("(".to_string());

    // parse arguments differenly for functions that are not main
    // since rust does not have arguments or return type for main
    if lexeme[1].get_token_type() != MAIN {

        // collect arguments
        while lexeme[lookahead].get_token_type() != RIGHT_BRACKET {
            lookahead += 1;
        }
        while head < lookahead {
            let l: Token = lexeme[head].clone();
            temp_lexeme.push(l);
            head += 1;
        }
        // parse arguments
        stream.append(&mut parse_arguments(&temp_lexeme));
        temp_lexeme.clear();

        stream.push(")".to_string());

        // parse return type
        if let Some(rust_type) = parse_type(lexeme[0].get_token_type() as i32) {
            if rust_type != "void" {
                stream.push("->".to_string());
                stream.push(rust_type);
            }
        }

        stream.push("{".to_string());
    }
    // declare argc and argv inside main, if required
    else {
        stream.push(")".to_string());
        stream.push("{".to_string());
        if lexeme[head].get_token_type() != RIGHT_BRACKET {
            unsafe {
                if strict == false {
                    stream.push(NO_STRICT.get_doc().to_string());
                    stream.push("let mut argv: Vec<_> = std::env::args().collect();".to_string());
                    stream.push("let mut argc = argv.len();".to_string());
                } else {
                    stream.push(STRICT.get_doc().to_string());

                    stream.push("let argv: Vec<_> = std::env::args().collect();".to_string());
                    stream.push("let argc = argv.len();".to_string());
                }
            }
        }

    }

    while lexeme[head].get_token_type() != LEFT_CBRACE {
        head += 1
    }
    head += 1;

    // collect function body
    // len - 1  so that '}' is excluded
    while head < lexeme.len() - 1 {
        let l: Token = lexeme[head].clone();
        temp_lexeme.push(l);
        head += 1;
    }
    // parse function body
    stream.append(&mut parse_program(&temp_lexeme));
    stream.push("}".to_string());
    stream
}


/**
 * parse-arguments:
 * parse c/c++ formal arguments in the function signature
 * into rust equivalent arguments
 */
fn parse_arguments(lexeme: &Vec<Token>) -> Vec<String> {
    let mut stream: Vec<String> = Vec::new();
    let mut head: usize = 0;
    while head < lexeme.len() {
        if lexeme[head].get_token_type() == COMMA {
            stream.push(",".to_string());
            head += 1;
            continue;
        }
        // push identifier
        stream.push(lexeme[head + 1].get_token_value()); //int f(int val)
        stream.push(":".to_string());

        // parse argument type
        if let Some(rust_type) = parse_type(lexeme[head].get_token_type() as i32) {
            stream.push(rust_type);
        }
        head += 2;
    }
    stream
}


/**
 * parse_declaration:
 * parse c/c++ declaration into rust
 * equivalent statements
 */
fn parse_declaration(lexeme: &Vec<Token>) -> Vec<String> {
    let mut stream: Vec<String> = Vec::new();

    let mut sym_tab: Vec<SymbolTable> = Vec::new();
    sym_tab.clear();
    let mut sym: SymbolTable = SymbolTable {
        typ: -1,
        id_name: "undefined_var".to_string(),
        is_assigned: false,
        is_ptr: false,
        assigned_val: "NONE".to_string(),
    };
    let mut head: usize = 1;
    //let sym_idx:usize=0;
    while head < lexeme.len() {

        match lexeme[head].get_token_type() {

            IDENTIFIER => sym.id_name = lexeme[head].get_token_value(),

            OP_ASSIGN => {
                sym.is_assigned = true;
                sym.assigned_val = String::from("");
                head += 1;
                let mut br = 0;


                if sym.is_ptr == true {
                    if lexeme[head].get_token_type() == NULL {
                        while lexeme[head].get_token_type() != SEMICOLON &&
                              lexeme[head].get_token_type() != COMMA {
                            head += 1;
                        }
                        sym.is_assigned = false;
                    } else {
                        head += 1;
                    }
                }
                while lexeme[head].get_token_type() != SEMICOLON &&
                      !(br == 0 && lexeme[head].get_token_type() == COMMA) {
                    if lexeme[head].get_token_type() == LEFT_BRACKET {
                        br += 1;
                    }
                    if lexeme[head].get_token_type() == RIGHT_BRACKET {
                        br -= 1;
                    }
                    sym.assigned_val.push_str(&lexeme[head].get_token_value());
                    head += 1;
                }
                continue;
            }

            SEMICOLON | COMMA => {
                // used enum value in the symbol table
                sym.typ = lexeme[0].get_token_type() as i32;
                sym_tab.push(sym.clone());
            }
            OP_MUL => {
                sym.is_ptr = true;
            }
            _ => {
                sym.assigned_val.push_str(&lexeme[head].get_token_value());
            }

        };
        head += 1;
    }

    unsafe {
        if strict == false {
            stream.push(NO_STRICT.get_doc().to_string());
        } else {
            stream.push(STRICT.get_doc().to_string());
        }
    }

    for i in &sym_tab {
        // get identifier
        //for declaration out of any blocks(global)
        unsafe {
            if strict == false {
                if IN_BLOCK_STMNT == true {
                    stream.push("let mut".to_string());
                } else {
                    stream.push("static mut".to_string());
                }
            } else {
                if IN_BLOCK_STMNT == true {
                    stream.push("let".to_string());
                } else {
                    stream.push("static".to_string());
                }
            }
        }
        stream.push(i.id_name.clone());
        stream.push(":".to_string());

        if i.is_ptr == true {
            stream.push("&".to_string());

            unsafe {
                if strict == false {
                    stream.push("mut".to_string());
                }

            }
        }
        // get the rust type
        if let Some(rust_type) = parse_type(i.typ) {
            stream.push(rust_type);
        } else {
            stream.push("UNKNOWN_TYPE".to_string());
        }

        // take care of assignment
        if i.is_assigned {
            stream.push("=".to_string());
            if i.is_ptr == true {
                stream.push("&".to_string());
            }
            unsafe {
                if strict == false && i.is_ptr == true {
                    stream.push("mut".to_string());
                }
            }
            stream.push((&i.assigned_val).to_string());

        }
        stream.push(";".to_string());
    }
    stream
}


/**
 * parse_if:
 * parse c/c++ if statements into rust
 * equivalent statements
 */
fn parse_if(lexeme: &Vec<Token>) -> Vec<String> {
    let mut stream: Vec<String> = Vec::new();
    let mut head: usize = 0;

    stream.push("if".to_string());
    stream.push("(".to_string());
    head += 1;

    //skip '('
    head += 1;

    // condition
    while lexeme[head].get_token_type() != RIGHT_BRACKET {
        stream.push(lexeme[head].get_token_value());
        head += 1;
    }
    head += 1;
    stream.push(")".to_string());
    stream.push("== true".to_string());
    stream.push("{".to_string());

    if lexeme[head].get_token_type() == LEFT_CBRACE {
        head += 1;
    }

    // collect if body
    let mut temp_lexeme: Vec<Token> = Vec::new();
    while head < lexeme.len() {
        let l: Token = lexeme[head].clone();
        temp_lexeme.push(l);
        head += 1;
    }
    // parse if body
    stream.append(&mut parse_program(&temp_lexeme));

    stream.push("}".to_string());
    stream
}


/**
 * parse_while:
 * parse c/c++ while statements into rust
 * equivalent statements
 */
fn parse_while(lexeme: &Vec<Token>) -> Vec<String> {
    let mut stream: Vec<String> = Vec::new();
    let mut head: usize = 0;
    let mut no_cond = false;
    head += 1;

    //skip '('
    head += 1;
    // condition
    let mut cond_stream: Vec<String> = Vec::new();
    while lexeme[head].get_token_type() != RIGHT_BRACKET {
        cond_stream.push(lexeme[head].get_token_value());
        head += 1;
    }
    if cond_stream.len() == 1 && (cond_stream[0] == "1" || cond_stream[0] == "true") {
        no_cond = true;
    }
    head += 1;

    if lexeme[head].get_token_type() == LEFT_CBRACE {
        head += 1;
    }

    // collect while body
    let mut temp_lexeme: Vec<Token> = Vec::new();
    while head < lexeme.len() {
        let l: Token = lexeme[head].clone();
        temp_lexeme.push(l);
        head += 1;
    }
    // parse while body
    let mut body_stream = &mut parse_program(&temp_lexeme);

    if no_cond == true {
        stream.push("loop".to_string());
    } else {
        stream.push("while".to_string());
        stream.push("(".to_string());
        stream.append(&mut cond_stream);
        stream.push(")".to_string());
        stream.push("== true".to_string());
    }
    stream.push("{".to_string());
    stream.append(&mut body_stream);

    stream.push("}".to_string());
    stream
}


/**
 * parse_dowhile:
 * parse c/c++ do while statements into rust
 * equivalent statements
 */
fn parse_dowhile(lexeme: &Vec<Token>) -> Vec<String> {
    let mut stream: Vec<String> = Vec::new();
    let mut temp_stream: Vec<String> = Vec::new();
    let mut head: usize = 0;
    let mut lookahead: usize;

    head += 2;
    lookahead = head;

    lookahead = skip_block(&lexeme, lookahead) - 1;
    // collect while body
    let mut temp_lexeme: Vec<Token> = Vec::new();
    while head < lookahead {
        let l: Token = lexeme[head].clone();
        temp_lexeme.push(l);
        head += 1;
    }
    // parse while body

    temp_stream.append(&mut parse_program(&temp_lexeme));
    temp_lexeme.clear();

    head += 3;
    if (lexeme[head].get_token_value() == "1" || lexeme[head].get_token_value() == "true") &&
       lexeme[head + 1].get_token_type() == RIGHT_BRACKET {
        stream.push("loop".to_string());
        stream.push("{".to_string());
        stream.append(&mut temp_stream);

        stream.push("}".to_string());
    } else {
        stream.push("while".to_string());
        stream.push("{".to_string());
        stream.append(&mut temp_stream);
        stream.push("(".to_string());
        while lexeme[head].get_token_type() != RIGHT_BRACKET {
            stream.push(lexeme[head].get_token_value());
            head += 1;
        }
        stream.push(")".to_string());
        stream.push("== true".to_string());
        stream.push("}".to_string());
        stream.push("{".to_string());
        stream.push("}".to_string());
    }
    stream.push(";".to_string());
    stream
}


fn parse_switch(lexeme: &Vec<Token>) -> Vec<String> {
    let mut head: usize = 2;
    let mut lookahead: usize = 2;
    let mut stream: Vec<String> = Vec::new();
    let mut temp_lexeme: Vec<Token> = Vec::new();

    stream.push("match".to_string());

    // find starting of switch block
    while lexeme[lookahead].get_token_type() != LEFT_CBRACE {
        lookahead += 1;
    }
    // {
    // move back to find the variable/result to be matched
    lookahead -= 1;
    // single variable
    if lookahead - head == 1 {
        stream.push(lexeme[lookahead - 1].get_token_value());
    }
    // expression
    else {
        while head < lookahead {
            let l: Token = lexeme[head].clone();
            temp_lexeme.push(l);
            head += 1;
        }
        head -= 1;
        stream.append(&mut parse_program(&temp_lexeme));


        temp_lexeme.clear();
    }
    // move forward to the starting of the block
    head += 3;
    stream.push("{".to_string());

    //head is at case
    lookahead = skip_block(&lexeme, head) - 1;
    while head < lookahead {
        let l: Token = lexeme[head].clone();
        temp_lexeme.push(l);
        head += 1;
    }
    stream.append(&mut parse_case(&temp_lexeme));
    stream.push("}".to_string());
    stream
}


fn parse_case(lexeme: &Vec<Token>) -> Vec<String> {
    let mut stream: Vec<String> = Vec::new();
    //head is at case
    let mut head: usize = 0;
    let mut lookahead: usize;
    let mut temp_lexeme: Vec<Token> = Vec::new();
    let mut def: bool = false;
    //look whether default case is handled for exaustive search
    while head < lexeme.len() {
        if lexeme[head].get_token_type() == KEYWORD_DEFAULT {
            stream.push("_".to_string());
            def = true;
        } else {
            head += 1; //head is at matching value
            stream.push(lexeme[head].get_token_value());
        }

        head += 1; // head is at :
        stream.push("=>".to_string());

        // either brace or no brace
        head += 1;
        if lexeme[head].get_token_type() == LEFT_CBRACE {
            head += 1;
            lookahead = skip_block(&lexeme, head) - 1;
        } else {
            lookahead = head;
            while lookahead < lexeme.len() && lexeme[lookahead].get_token_type() != KEYWORD_CASE &&
                  lexeme[lookahead].get_token_type() != KEYWORD_DEFAULT {
                lookahead += 1;
            }
        }
        while head < lookahead {
            let l: Token = lexeme[head].clone();
            temp_lexeme.push(l);
            head += 1;
        }
        stream.push("{".to_string());
        stream.append(&mut parse_program(&temp_lexeme));
        stream.push("}".to_string());


        if head < lexeme.len() && lexeme[head].get_token_type() == RIGHT_CBRACE {
            head += 1;
        }
        temp_lexeme.clear();
    }
    if def == false {
        stream.push("_".to_string());
        stream.push("=>".to_string());
        stream.push("{".to_string());
        stream.push("}".to_string());
    }
    stream
}


/**
 * parse_for:
 * parse c/c++ do while statements into rust
 * equivalent statements
 *
 * Identify infinite loops and replace for with loop{} 
 */
fn parse_for(lexeme: &Vec<Token>) -> Vec<String> {
    let mut stream: Vec<String> = Vec::new();
    let mut head: usize = 0;
    let mut lookahead: usize;
    let mut temp_lexeme: Vec<Token> = Vec::new();

    while lexeme[head].get_token_type() != LEFT_BRACKET {
        head += 1;
    }
    head += 1;
    lookahead = head;

    //for (int i =0; )
    let decl: bool = if lexeme[head].get_base_type() == BASE_DATATYPE {
        true
    } else {
        false
    };
    // let mut no_init:bool; //no initialization
    let mut no_cond: bool = false; //if no condition to terminate
    let mut no_updation: bool = false; //no inc/dec of loop counter



    let mut body: Vec<String> = Vec::new();
    let mut updation: Vec<String> = Vec::new();
    let mut term_cond: Vec<String> = Vec::new();
    // initial assignment
    lookahead = skip_stmt(&lexeme, lookahead);

    //incase of initialization expressio for (;i<10;i++) ; common case
    if head + 1 < lookahead {
        while head < lookahead {
            let l: Token = lexeme[head].clone();
            temp_lexeme.push(l);
            head += 1;
        }

        if decl == true {
            stream.append(&mut parse_declaration(&temp_lexeme));
        } else {
            stream.append(&mut parse_assignment(&temp_lexeme));
        }
    } else {
        head += 1;
        // no_init = true;
    }
    temp_lexeme.clear();

    // terminating condition
    lookahead = skip_stmt(&lexeme, lookahead);


    if head + 1 < lookahead {
        while head < lookahead - 1 {
            term_cond.push(lexeme[head].get_token_value());
            head += 1;
        }
    } else {
        no_cond = true;
    }
    head += 1;
    temp_lexeme.clear();

    lookahead = head;
    // update expression
    while lexeme[lookahead].get_token_type() != RIGHT_BRACKET {
        let l: Token = lexeme[lookahead].clone();
        temp_lexeme.push(l);
        lookahead += 1;
    }
    //no_updation
    if head == lookahead {
        no_updation = true;
    } else {

        temp_lexeme.push(Token::new(String::from(";"), BASE_NONE, SEMICOLON, 0, 0));
        updation.append(&mut parse_program(&temp_lexeme));
        temp_lexeme.clear();
    }
    head = lookahead;
    head += 1;
    if lexeme[head].get_token_type() == LEFT_CBRACE {
        head += 1;
        lookahead = skip_block(&lexeme, head);
    } else {
        lookahead = skip_stmt(&lexeme, head);
    }

    // lookahead = skip_block(&lexeme, lookahead);
    while head < lookahead {
        let l: Token = lexeme[head].clone();
        temp_lexeme.push(l);
        head += 1;
    }
    body.append(&mut parse_program(&temp_lexeme));

    if no_cond == true {
        stream.push("loop".to_string());
    } else {
        stream.push("while".to_string());
        stream.append(&mut term_cond); //append termianating condition
    }
    stream.push("{".to_string());
    stream.append(&mut body);
    if no_updation != true {
        stream.append(&mut updation);
    }

    stream.push("}".to_string());

    stream
}

/**
 * fn parse_type:
 * takes the integer value of type Type
 * returns either the equivalent Rust type as a string or
 * None, if does not correspond to any c/c++ type
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
        7 => Some("void".to_string()),
        _ => None,
    }
}


/* parse_assignment:
 * parse c/c++ assignment statements into rust equivalent code
 * compound assignments must be converted to declarations
 * as rust doesnt support compound assignment
 */
fn parse_assignment(lexeme: &Vec<Token>) -> Vec<String> {
    let mut stream: Vec<String> = Vec::new();
    // let mut lookahead = lexeme.len();
    let mut thead: usize = 2;
    let mut lexeme1: Vec<Token> = Vec::new();

    let mut n = 2;
    let m = 3;

    let mut tstream: Vec<String> = Vec::new();
    // if lexeme[0].get_token_type() == OP_MUL {println!("tasdbvjhbhsb "); m+=1; n+=1;thead+=1;}
    if lexeme[n].get_base_type() == BASE_UNOP {

        while lexeme[thead].get_token_type() != SEMICOLON {
            lexeme1.push(lexeme[thead].clone());
            thead += 1;
        }
        lexeme1.push(lexeme[thead].clone());
        stream.push(lexeme[0].get_token_value());
        stream.push(lexeme[1].get_token_value());
        stream.append(&mut parse_expr(&lexeme1));
    } else if lexeme[m].get_base_type() == BASE_UNOP {

        while lexeme[thead].get_token_type() != SEMICOLON {
            lexeme1.push(lexeme[thead].clone());
            thead += 1;
        }
        lexeme1.push(lexeme[thead].clone());
        stream.push(lexeme[0].get_token_value());
        stream.push(lexeme[1].get_token_value());
        stream.append(&mut parse_expr(&lexeme1));
    } else if lexeme[m].get_base_type() == BASE_BINOP {
        while lexeme[thead].get_token_type() != SEMICOLON {
            lexeme1.push(lexeme[thead].clone());
            thead += 1;
        }
        lexeme1.push(lexeme[thead].clone());
        stream.push(lexeme[0].get_token_value());
        stream.push(lexeme[1].get_token_value());
        stream.append(&mut parse_expr(&lexeme1));
    } else if lexeme[n].get_token_type() == OP_BITAND {
        stream.push(lexeme[0].get_token_value());
        stream.push(lexeme[1].get_token_value());

        while lexeme[thead].get_token_type() != SEMICOLON {
            stream.push(lexeme[thead].get_token_value());
            thead += 1;
        }
    } else {

        if lexeme[m].get_token_type() == OP_ASSIGN {
            while lexeme[thead].get_token_type() != SEMICOLON &&
                  lexeme[thead].get_token_type() != COMMA {
                lexeme1.push(lexeme[thead].clone());
                thead += 1;
            }
            lexeme1.push(lexeme[thead].clone());
            stream.append(&mut parse_program(&lexeme1));
        }
        stream.push(lexeme[0].get_token_value());
        stream.push(lexeme[1].get_token_value());
        if lexeme[n].get_base_type() == BASE_UNOP {
            stream.push(lexeme[m].get_token_value());
        } else {
            stream.push(lexeme[n].get_token_value());
            n += 1;
            if lexeme[n].get_token_type() == LEFT_BRACKET ||
               lexeme[n].get_token_type() == LEFT_SBRACKET {
                while lexeme[n].get_token_type() != SEMICOLON {
                    stream.push(lexeme[n].get_token_value());
                    n += 1;
                }
            }
            stream.push(";".to_string());
        }

    }
    if tstream.len() > 0 {
        stream.append(&mut tstream);
    }
    stream
}


/* parse_expr:
 * parse c/c++ expression statements into rust equivalent code
 */
fn parse_expr(lexeme: &Vec<Token>) -> Vec<String> {
    let mut stream: Vec<String> = Vec::new();
    // let mut lookahead = lexeme.len();
    let mut tstream: Vec<String> = Vec::new();
    let mut thead: usize = 0;

    let mut prev_id = " ".to_string();
    let mut typ = OTHER;
    //a=b+c++;

    while lexeme[thead].get_token_type() != SEMICOLON {

        if lexeme[thead].get_base_type() == BASE_UNOP {
            //incase of post
            if typ == IDENTIFIER {
                tstream.push(prev_id.clone());
                tstream.push(match lexeme[thead].get_token_type() {
                    OP_INC => "+=1".to_string(),
                    OP_DEC => "-=1".to_string(),
                    _ => " ;".to_string(),
                });
                tstream.push(";".to_string());

                thead + 1;
                //continue;
            }
            // incase of pre
            else {
                stream.push("(".to_string());
                stream.push(lexeme[thead + 1].get_token_value());
                stream.push(match lexeme[thead].get_token_type() {
                    OP_INC => "+=1".to_string(),
                    OP_DEC => "-=1".to_string(),
                    _ => " ;".to_string(),
                });
                stream.push(")".to_string());
                thead += 1;
            }


        } else {
            stream.push(lexeme[thead].get_token_value());
        }

        typ = lexeme[thead].get_token_type();
        prev_id = lexeme[thead].get_token_value();

        thead += 1;
    }
    stream.push(";".to_string());
    if tstream.len() > 0 {
        stream.append(&mut tstream);
    }
    stream
}


fn parse_array_declaration(lexeme: &Vec<Token>) -> Vec<String> {
    let mut stream: Vec<String> = Vec::new();
    let mut typ: String = " ".to_string();

    //int a[10];
    if let Some(t) = parse_type(lexeme[0].get_token_type() as i32) {
        typ = t;
    }
    unsafe {
        if strict == true {
            stream.push(STRICT.get_doc().to_string());
            stream.push("let".to_string());
        } else {
            stream.push(NO_STRICT.get_doc().to_string());
            stream.push("let mut".to_string());

        }
    }
    let mut head = 0;
    stream.push(lexeme[head + 1].get_token_value());
    stream.push(":".to_string());
    stream.push("[".to_string() + &typ[..] + ";" + &lexeme[head + 3].get_token_value()[..] + "]");
    head = 5;
    let mut lookahead = head;
    while lexeme[lookahead].get_token_type() != SEMICOLON {
        lookahead += 1;
    }
    let mut temp_lexeme: Vec<Token> = Vec::new();
    if lexeme[head].get_token_type() == COMMA {
        temp_lexeme.push(lexeme[0].clone());
        //move to next
        head += 1;
        while lexeme[head].get_token_type() != SEMICOLON {
            temp_lexeme.push(lexeme[head].clone());
            head += 1;
        }
        stream.push(";".to_string());
        temp_lexeme.push(lexeme[head].clone());
        stream.append(&mut parse_program(&temp_lexeme))
    } else if lexeme[head].get_token_type() == OP_ASSIGN {
        while lexeme[head].get_token_type() != SEMICOLON &&
              lexeme[head].get_token_type() != RIGHT_CBRACE {
            stream.push(match lexeme[head].get_token_type() {
                LEFT_CBRACE => "[".to_string(),

                _ => lexeme[head].get_token_value(),
            });
            head += 1;
        }
        stream.push("]".to_string());
        stream.push(";".to_string());
    } else {
        stream.push(";".to_string());
    }

    stream
}

// not tested
fn parse_struct(lexeme: &Vec<Token>, mut structmem: &mut Vec<StructMem>) -> Vec<String> {
    let mut stream: Vec<String> = Vec::new();
    let mut head: usize = 0;
    stream.push(lexeme[head].get_token_value()); //push the keyword parse_struct
    head += 1;
    //push the struct id_name
    stream.push(lexeme[head].get_token_value()); //push the keyword parse_struct
    let name = lexeme[head].get_token_value();
    stream.push("{".to_string());
    head += 2;
    let mut temp_lexeme: Vec<Token> = Vec::new();
    while lexeme[head].get_token_type() != RIGHT_CBRACE {
        while lexeme[head].get_token_type() != SEMICOLON {
            temp_lexeme.push(lexeme[head].clone());
            head += 1
        }
        temp_lexeme.push(lexeme[head].clone());
        head += 1;
        stream.append(&mut parse_struct_inbody_decl(&temp_lexeme, &mut structmem, &name));
        temp_lexeme.clear();
    }
    stream.push(lexeme[head].get_token_value() + "\n");


    stream
}

// not tested
fn parse_struct_inbody_decl(lexeme: &Vec<Token>,
                            struct_mem: &mut Vec<StructMem>,
                            name: &String)
                            -> Vec<String> {
    let mut stream: Vec<String> = Vec::new();

    //push the identifier
    stream.push(lexeme[1].get_token_value());
    stream.push(":".to_string());
    let mut struct_memt = StructMem {
        identifier: "NONE".to_string(),
        typ: 0,
        name: name.clone(),
    };

    if let Some(rust_type) = parse_type(lexeme[0].get_token_type() as i32) {
        stream.push(rust_type);
        struct_memt.typ = lexeme[0].get_token_type() as i32;
        struct_memt.identifier = lexeme[1].get_token_value();
    }
    struct_mem.push(struct_memt);
    stream.push(",".to_string());
    stream
}

// not tested
fn parse_struct_decl(lexeme: &Vec<Token>, struct_table: &Vec<StructMem>) -> Vec<String> {
    let mut stream: Vec<String> = Vec::new();

    stream.push(STRUCT_INIT.get_doc().to_string());
    stream.push("let".to_string());
    let mut head = 1;
    //struct FilePointer fp;
    let struct_name = lexeme[head].get_token_value();
    head += 1;
    stream.push(lexeme[head].get_token_value()); //push the identifer => let a
    stream.push("=".to_string());
    stream.push(struct_name.clone());
    stream.push("{".to_string());

    for row in struct_table {
        if row.name == struct_name {
            stream.push(row.identifier.clone());
            stream.push(":".to_string());
            stream.push(get_default_value_for(row.typ));
            stream.push(",".to_string());
        }
    }
    stream.push("};".to_string());

    stream
}

// not tested
fn parse_class(lexeme: &Vec<Token>, mut structmem: &mut Vec<StructMem>) -> Vec<String> {
    let mut stream: Vec<String> = Vec::new();
    let mut head: usize = 0;
    let mut method_stream: Vec<String> = Vec::new();
    stream.push("struct".to_string()); //push the keyword struct
    head += 1;
    //push the struct id_name
    let class_name = lexeme[head].get_token_value();
    stream.push(class_name.clone()); //push the class name
    let name = lexeme[head].get_token_value();
    stream.push("{".to_string());
    head += 2;
    let mut modifier: String = " ".to_string();
    let mut temp_lexeme: Vec<Token> = Vec::new();
    let mut tstream: Vec<String> = Vec::new();

    while lexeme[head].get_token_type() != RIGHT_CBRACE &&
          lexeme[head + 1].get_token_type() != SEMICOLON {
        match lexeme[head].get_type() {
            (BASE_MODIFIER, _) => {
                match lexeme[head].get_token_type() {
                    KEYWORD_PUBLIC => {
                        head += 2;
                        modifier = "pub".to_string();
                    }
                    KEYWORD_PROTECTED |
                    KEYWORD_PRIVATE => {
                        head += 2;
                        modifier = "".to_string();
                    }
                    _ => {}
                };
            }
            (_, IDENTIFIER) => {
                if lexeme[head].get_token_value() == class_name {
                    tstream.push(CONSTRUCTOR.get_doc().to_string());
                    let mut lookahead = head;
                    while lexeme[lookahead].get_token_type() != LEFT_CBRACE {
                        lookahead += 1;
                    }
                    lookahead += 1;
                    lookahead = skip_block(lexeme, lookahead);
                    while head < lookahead {
                        tstream.push(lexeme[head].get_token_value());
                        head += 1;
                    }
                    tstream.push("\n **/\n".to_string());
                    continue;
                }
            }

            _ => {}
        }

        if lexeme[head + 2].get_token_type() == LEFT_BRACKET {

            while lexeme[head].get_token_type() != RIGHT_CBRACE {

                temp_lexeme.push(lexeme[head].clone());
                head += 1;
            }
            temp_lexeme.push(lexeme[head].clone());
            head += 1;
            method_stream.append(&mut parse_method_decl(&temp_lexeme, &modifier));
            temp_lexeme.clear();

        } else {
            while lexeme[head].get_token_type() != RIGHT_CBRACE &&
                  lexeme[head].get_base_type() != BASE_MODIFIER {
                while lexeme[head].get_token_type() != SEMICOLON {
                    temp_lexeme.push(lexeme[head].clone());
                    head += 1
                }
                temp_lexeme.push(lexeme[head].clone());
                head += 1;
                stream.append(&mut parse_class_inbody_decl(&temp_lexeme,
                                                           &mut structmem,
                                                           &name,
                                                           &modifier));
                temp_lexeme.clear();
            }
        }

    }
    stream.push(lexeme[head].get_token_value());
    stream.push("\n\n/**Method declarations are wrapped inside the impl block \
    \n * Which implements the corresponding structure\
    \n **/\n"
        .to_string());
    stream.push("impl".to_string());
    stream.push(name.clone());
    stream.push("{\n".to_string());
    if tstream.len() > 0 {
        stream.append(&mut tstream);
    }
    stream.append(&mut method_stream);

    stream.push("}\n".to_string());
    stream
}

// not tested
fn parse_method_decl(lexeme: &Vec<Token>, modifier: &String) -> Vec<String> {
    let mut temp_lexeme: Vec<Token> = Vec::new();
    let mut head: usize = 3;
    let mut lookahead: usize = head;
    let mut stream: Vec<String> = Vec::new();
    if modifier.len() > 1 {
        stream.push(modifier.clone());
    }
    stream.push("fn".to_string());
    stream.push(lexeme[1].get_token_value());
    stream.push("(".to_string());
    stream.push("&self".to_string()); //first argument of method must be self, for sefety we consider reference/borrow
    // parse arguments differenly for functions that are not main
    // collect arguments
    while lexeme[lookahead].get_token_type() != RIGHT_BRACKET {
        lookahead += 1;
    }
    if head < lookahead {
        stream.push(",".to_string());
    }
    while head < lookahead {
        let l: Token = lexeme[head].clone();
        temp_lexeme.push(l);
        head += 1;
    }

    // parse arguments
    stream.append(&mut parse_arguments(&temp_lexeme));
    temp_lexeme.clear();

    stream.push(")".to_string());

    // parse return type
    if let Some(rust_type) = parse_type(lexeme[0].get_token_type() as i32) {
        if rust_type != "void" {
            stream.push("->".to_string());
            stream.push(rust_type);
        }
    }

    stream.push("{".to_string());
    while lexeme[head].get_token_type() != LEFT_CBRACE {
        head += 1
    }
    head += 1;

    // collect function body
    // len - 1  so that '}' is excluded
    while head < lexeme.len() - 1 {
        let l: Token = lexeme[head].clone();
        temp_lexeme.push(l);
        head += 1;
    }
    // parse function body
    stream.append(&mut parse_program(&temp_lexeme));
    stream.push("}".to_string());
    stream
}

// not tested
fn parse_class_inbody_decl(lexeme: &Vec<Token>,
                           struct_mem: &mut Vec<StructMem>,
                           name: &String,
                           modifier: &String)
                           -> Vec<String> {
    let mut stream: Vec<String> = Vec::new();

    //push the identifier
    if modifier.len() > 1 {
        stream.push(modifier.clone());
    }
    stream.push(lexeme[1].get_token_value());
    stream.push(":".to_string());
    let mut struct_memt = StructMem {
        identifier: "NONE".to_string(),
        typ: 0,
        name: name.clone(),
    };

    if let Some(rust_type) = parse_type(lexeme[0].get_token_type() as i32) {
        stream.push(rust_type);
        struct_memt.typ = lexeme[0].get_token_type() as i32;
        struct_memt.identifier = lexeme[1].get_token_value();
    }
    struct_mem.push(struct_memt);
    stream.push(",".to_string());
    stream
}

// not tested
fn parse_class_decl(lexeme: &Vec<Token>, struct_table: &Vec<StructMem>) -> Vec<String> {
    let mut stream: Vec<String> = Vec::new();

    stream.push(STRUCT_INIT.get_doc().to_string());
    stream.push("let".to_string());
    let mut head = 0;
    //struct FilePointer fp;
    let struct_name = lexeme[head].get_token_value();
    head += 1;
    stream.push(lexeme[head].get_token_value()); //push the identifer => let a
    stream.push("=".to_string());
    stream.push(struct_name.clone());
    stream.push("{".to_string());

    for row in struct_table {
        if row.name == struct_name {
            stream.push(row.identifier.clone());
            stream.push(":".to_string());
            stream.push(get_default_value_for(row.typ));
            stream.push(",".to_string());
        }
    }
    stream.push("};".to_string());

    stream
}

fn get_default_value_for(c_type: i32) -> String {
    match c_type {
        0 => "0i32".to_string(),
        1 => "0i16".to_string(),
        2 => "0i64".to_string(),
        3 => "0.0f32".to_string(),
        4 => "0.0f64".to_string(),
        5 => "'_'".to_string(),
        6 => "false".to_string(),
        _ => "_".to_string(),
    }
}


// if
#[test]
fn test_parse_if_braces() {
    let tok_vector = vec![Token::new(String::from("if"), BASE_NONE, KEYWORD_IF, 0, 0),
                          Token::new(String::from("("), BASE_NONE, LEFT_BRACKET, 0, 0),
                          Token::new(String::from("a"), BASE_NONE, IDENTIFIER, 0, 0),
                          Token::new(String::from("=="), BASE_BINOP, OP_EQU, 0, 0),
                          Token::new(String::from("a"), BASE_NONE, IDENTIFIER, 0, 0),
                          Token::new(String::from(")"), BASE_NONE, RIGHT_BRACKET, 0, 0),
                          Token::new(String::from("{"), BASE_NONE, LEFT_CBRACE, 0, 0),
                          Token::new(String::from("/*Do something here*/"),
                                     BASE_COMMENT,
                                     COMMENT_MULTI,
                                     0,
                                     0),
                          Token::new(String::from("}"), BASE_NONE, RIGHT_CBRACE, 0, 0)];
    let stream =
        vec!["if", "(", "a", "==", "a", ")", "== true", "{", "/*Do something here*/\n", "}"];

    assert_eq!(stream, parse_if(&tok_vector));
}

#[test]
fn test_parse_if_braces_nesting() {
    let tok_vector = vec![Token::new(String::from("if"), BASE_NONE, KEYWORD_IF, 0, 0),
                          Token::new(String::from("("), BASE_NONE, LEFT_BRACKET, 0, 0),
                          Token::new(String::from("a"), BASE_NONE, IDENTIFIER, 0, 0),
                          Token::new(String::from(">"), BASE_BINOP, OP_GT, 0, 0),
                          Token::new(String::from("2"), BASE_VALUE, NUM_INT, 0, 0),
                          Token::new(String::from(")"), BASE_NONE, RIGHT_BRACKET, 0, 0),
                          Token::new(String::from("{"), BASE_NONE, LEFT_CBRACE, 0, 0),
                          Token::new(String::from("/*Do something here*/"),
                                     BASE_COMMENT,
                                     COMMENT_MULTI,
                                     0,
                                     0),
                          Token::new(String::from("if"), BASE_NONE, KEYWORD_IF, 0, 0),
                          Token::new(String::from("("), BASE_NONE, LEFT_BRACKET, 0, 0),
                          Token::new(String::from("a"), BASE_NONE, IDENTIFIER, 0, 0),
                          Token::new(String::from("<"), BASE_BINOP, OP_LT, 0, 0),
                          Token::new(String::from("4"), BASE_VALUE, NUM_INT, 0, 0),
                          Token::new(String::from(")"), BASE_NONE, RIGHT_BRACKET, 0, 0),
                          Token::new(String::from("{"), BASE_NONE, LEFT_CBRACE, 0, 0),
                          Token::new(String::from("b"), BASE_NONE, IDENTIFIER, 0, 0),
                          Token::new(String::from("="), BASE_NONE, OP_ASSIGN, 0, 0),
                          Token::new(String::from("53"), BASE_VALUE, NUM_INT, 0, 0),
                          Token::new(String::from(";"), BASE_VALUE, SEMICOLON, 0, 0),
                          Token::new(String::from("}"), BASE_NONE, RIGHT_CBRACE, 0, 0),
                          Token::new(String::from("b"), BASE_NONE, IDENTIFIER, 0, 0),
                          Token::new(String::from("="), BASE_NONE, OP_ASSIGN, 0, 0),
                          Token::new(String::from("72"), BASE_VALUE, NUM_INT, 0, 0),
                          Token::new(String::from(";"), BASE_NONE, SEMICOLON, 0, 0),
                          Token::new(String::from("}"), BASE_NONE, RIGHT_CBRACE, 0, 0)];
    let stream = vec!["if",
                      "(",
                      "a",
                      ">",
                      "2",
                      ")",
                      "== true",
                      "{",
                      "/*Do something here*/\n",
                      "if",
                      "(",
                      "a",
                      "<",
                      "4",
                      ")",
                      "== true",
                      "{",
                      "b",
                      "=",
                      "53",
                      ";",
                      "}",
                      "b",
                      "=",
                      "72",
                      ";",
                      "}"];

    assert_eq!(stream, parse_if(&tok_vector));
}

#[test]
fn test_parse_if_no_braces() {
    let tok_vector = vec![Token::new(String::from("if"), BASE_NONE, KEYWORD_IF, 0, 0),
                          Token::new(String::from("("), BASE_NONE, LEFT_BRACKET, 0, 0),
                          Token::new(String::from("a"), BASE_NONE, IDENTIFIER, 0, 0),
                          Token::new(String::from("=="), BASE_BINOP, OP_EQU, 0, 0),
                          Token::new(String::from("a"), BASE_NONE, IDENTIFIER, 0, 0),
                          Token::new(String::from(")"), BASE_NONE, RIGHT_BRACKET, 0, 0),
                          Token::new(String::from("a"), BASE_NONE, IDENTIFIER, 0, 0),
                          Token::new(String::from("="), BASE_NONE, OP_ASSIGN, 0, 0),
                          Token::new(String::from("5"), BASE_VALUE, NUM_INT, 0, 0),
                          Token::new(String::from(";"), BASE_NONE, SEMICOLON, 0, 0)];
    let stream = vec!["if", "(", "a", "==", "a", ")", "== true", "{", "a", "=", "5", ";", "}"];

    assert_eq!(stream, parse_if(&tok_vector));
}

#[test]
fn test_parse_if_no_braces_nesting() {
    let tok_vector = vec![Token::new(String::from("if"), BASE_NONE, KEYWORD_IF, 0, 0),
                          Token::new(String::from("("), BASE_NONE, LEFT_BRACKET, 0, 0),
                          Token::new(String::from("a"), BASE_NONE, IDENTIFIER, 0, 0),
                          Token::new(String::from(">"), BASE_BINOP, OP_GT, 0, 0),
                          Token::new(String::from("2"), BASE_VALUE, NUM_INT, 0, 0),
                          Token::new(String::from(")"), BASE_NONE, RIGHT_BRACKET, 0, 0),
                          Token::new(String::from("if"), BASE_NONE, KEYWORD_IF, 0, 0),
                          Token::new(String::from("("), BASE_NONE, LEFT_BRACKET, 0, 0),
                          Token::new(String::from("a"), BASE_NONE, IDENTIFIER, 0, 0),
                          Token::new(String::from("<"), BASE_BINOP, OP_LT, 0, 0),
                          Token::new(String::from("4"), BASE_VALUE, NUM_INT, 0, 0),
                          Token::new(String::from(")"), BASE_NONE, RIGHT_BRACKET, 0, 0),
                          Token::new(String::from("b"), BASE_NONE, IDENTIFIER, 0, 0),
                          Token::new(String::from("="), BASE_NONE, OP_ASSIGN, 0, 0),
                          Token::new(String::from("53"), BASE_VALUE, NUM_INT, 0, 0),
                          Token::new(String::from(","), BASE_NONE, COMMA, 0, 0),
                          Token::new(String::from("b"), BASE_NONE, IDENTIFIER, 0, 0),
                          Token::new(String::from("="), BASE_NONE, OP_ASSIGN, 0, 0),
                          Token::new(String::from("72"), BASE_VALUE, NUM_INT, 0, 0),
                          Token::new(String::from(";"), BASE_NONE, SEMICOLON, 0, 0)];
    let stream = vec!["if", "(", "a", ">", "2", ")", "== true", "{", "if", "(", "a", "<", "4",
                      ")", "== true", "{", "b", "=", "53", ";", "b", "=", "72", ";", "}", "}"];

    assert_eq!(stream, parse_if(&tok_vector));
}

#[test]
fn test_parse_if_no_braces_inside_braces() {
    let tok_vector = vec![Token::new(String::from("if"), BASE_NONE, KEYWORD_IF, 0, 0),
                          Token::new(String::from("("), BASE_NONE, LEFT_BRACKET, 0, 0),
                          Token::new(String::from("a"), BASE_NONE, IDENTIFIER, 0, 0),
                          Token::new(String::from(">"), BASE_BINOP, OP_GT, 0, 0),
                          Token::new(String::from("2"), BASE_VALUE, NUM_INT, 0, 0),
                          Token::new(String::from(")"), BASE_NONE, RIGHT_BRACKET, 0, 0),
                          Token::new(String::from("{"), BASE_NONE, LEFT_CBRACE, 0, 0),
                          Token::new(String::from("/*Do something here*/"),
                                     BASE_COMMENT,
                                     COMMENT_MULTI,
                                     0,
                                     0),
                          Token::new(String::from("if"), BASE_NONE, KEYWORD_IF, 0, 0),
                          Token::new(String::from("("), BASE_NONE, LEFT_BRACKET, 0, 0),
                          Token::new(String::from("a"), BASE_NONE, IDENTIFIER, 0, 0),
                          Token::new(String::from("<"), BASE_BINOP, OP_LT, 0, 0),
                          Token::new(String::from("4"), BASE_VALUE, NUM_INT, 0, 0),
                          Token::new(String::from(")"), BASE_NONE, RIGHT_BRACKET, 0, 0),
                          Token::new(String::from("b"), BASE_NONE, IDENTIFIER, 0, 0),
                          Token::new(String::from("="), BASE_NONE, OP_ASSIGN, 0, 0),
                          Token::new(String::from("53"), BASE_VALUE, NUM_INT, 0, 0),
                          Token::new(String::from(","), BASE_VALUE, COMMA, 0, 0),
                          Token::new(String::from("b"), BASE_NONE, IDENTIFIER, 0, 0),
                          Token::new(String::from("="), BASE_NONE, OP_ASSIGN, 0, 0),
                          Token::new(String::from("72"), BASE_VALUE, NUM_INT, 0, 0),
                          Token::new(String::from(";"), BASE_NONE, SEMICOLON, 0, 0),
                          Token::new(String::from("a"), BASE_NONE, IDENTIFIER, 0, 0),
                          Token::new(String::from("="), BASE_NONE, OP_ASSIGN, 0, 0),
                          Token::new(String::from("1"), BASE_VALUE, NUM_INT, 0, 0),
                          Token::new(String::from(";"), BASE_NONE, SEMICOLON, 0, 0),
                          Token::new(String::from("}"), BASE_NONE, RIGHT_CBRACE, 0, 0)];
    let stream = vec!["if",
                      "(",
                      "a",
                      ">",
                      "2",
                      ")",
                      "== true",
                      "{",
                      "/*Do something here*/\n",
                      "if",
                      "(",
                      "a",
                      "<",
                      "4",
                      ")",
                      "== true",
                      "{",
                      "b",
                      "=",
                      "53",
                      ";",
                      "b",
                      "=",
                      "72",
                      ";",
                      "}",
                      "a",
                      "=",
                      "1",
                      ";",
                      "}"];

    assert_eq!(stream, parse_if(&tok_vector));
}

#[test]
fn test_parse_if_else_ladder() {
    let tok_vector = vec![Token::new(String::from("if"), BASE_NONE, KEYWORD_IF, 0, 0),
                          Token::new(String::from("("), BASE_NONE, LEFT_BRACKET, 0, 0),
                          Token::new(String::from("a"), BASE_NONE, IDENTIFIER, 0, 0),
                          Token::new(String::from("=="), BASE_BINOP, OP_EQU, 0, 0),
                          Token::new(String::from("b"), BASE_NONE, IDENTIFIER, 0, 0),
                          Token::new(String::from(")"), BASE_NONE, RIGHT_BRACKET, 0, 0),
                          Token::new(String::from("{"), BASE_NONE, LEFT_CBRACE, 0, 0),
                          Token::new(String::from("/*Do something here*/"),
                                     BASE_COMMENT,
                                     COMMENT_MULTI,
                                     0,
                                     0),
                          Token::new(String::from("}"), BASE_NONE, RIGHT_CBRACE, 0, 0),
                          Token::new(String::from("else"), BASE_NONE, KEYWORD_ELSE, 0, 0),
                          Token::new(String::from("if"), BASE_NONE, KEYWORD_IF, 0, 0),
                          Token::new(String::from("("), BASE_NONE, LEFT_BRACKET, 0, 0),
                          Token::new(String::from("a"), BASE_NONE, IDENTIFIER, 0, 0),
                          Token::new(String::from("=="), BASE_BINOP, OP_EQU, 0, 0),
                          Token::new(String::from("c"), BASE_NONE, IDENTIFIER, 0, 0),
                          Token::new(String::from(")"), BASE_NONE, RIGHT_BRACKET, 0, 0),
                          Token::new(String::from("c"), BASE_NONE, IDENTIFIER, 0, 0),
                          Token::new(String::from("++"), BASE_UNOP, OP_INC, 0, 0),
                          Token::new(String::from(";"), BASE_NONE, SEMICOLON, 0, 0),
                          Token::new(String::from("else"), BASE_NONE, KEYWORD_ELSE, 0, 0),
                          Token::new(String::from("if"), BASE_NONE, KEYWORD_IF, 0, 0),
                          Token::new(String::from("("), BASE_NONE, LEFT_BRACKET, 0, 0),
                          Token::new(String::from("a"), BASE_NONE, IDENTIFIER, 0, 0),
                          Token::new(String::from("=="), BASE_BINOP, OP_EQU, 0, 0),
                          Token::new(String::from("d"), BASE_NONE, IDENTIFIER, 0, 0),
                          Token::new(String::from(")"), BASE_NONE, RIGHT_BRACKET, 0, 0),
                          Token::new(String::from("{"), BASE_NONE, LEFT_CBRACE, 0, 0),
                          Token::new(String::from("d"), BASE_NONE, IDENTIFIER, 0, 0),
                          Token::new(String::from("--"), BASE_UNOP, OP_DEC, 0, 0),
                          Token::new(String::from(";"), BASE_NONE, SEMICOLON, 0, 0),
                          Token::new(String::from("}"), BASE_NONE, RIGHT_CBRACE, 0, 0),
                          Token::new(String::from("else"), BASE_NONE, KEYWORD_ELSE, 0, 0),
                          Token::new(String::from("{"), BASE_NONE, LEFT_CBRACE, 0, 0),
                          Token::new(String::from("a"), BASE_NONE, IDENTIFIER, 0, 0),
                          Token::new(String::from("--"), BASE_UNOP, OP_DEC, 0, 0),
                          Token::new(String::from(";"), BASE_NONE, SEMICOLON, 0, 0),
                          Token::new(String::from("}"), BASE_NONE, RIGHT_CBRACE, 0, 0)];
    let stream = vec!["if",
                      "(",
                      "a",
                      "==",
                      "b",
                      ")",
                      "== true",
                      "{",
                      "/*Do something here*/\n",
                      "}",
                      "else",
                      "if",
                      "(",
                      "a",
                      "==",
                      "c",
                      ")",
                      "== true",
                      "{",
                      "c",
                      "+=1",
                      ";",
                      "}",
                      "else",
                      "if",
                      "(",
                      "a",
                      "==",
                      "d",
                      ")",
                      "== true",
                      "{",
                      "d",
                      "-=1",
                      ";",
                      "}",
                      "else",
                      "{",
                      "a",
                      "-=1",
                      ";",
                      "}"];

    assert_eq!(stream, parse_program(&tok_vector));
}

// variable declaration
#[test]
fn test_parse_declaration_static() {
    let doc = STRICT;

    let tok_vector = vec![Token::new(String::from("char"), BASE_DATATYPE, PRIMITIVE_CHAR, 0, 0),
                          Token::new(String::from("a"), BASE_NONE, IDENTIFIER, 0, 0),
                          //   Token::new(String::from("="), BASE_NONE, OP_ASSIGN, 0, 0),
                          //   Token::new(String::from("5"), BASE_VALUE, NUM_INT, 0, 0),
                          Token::new(String::from(";"), BASE_NONE, SEMICOLON, 0, 0)];
    let stream = vec![doc.get_doc(), "static", "a", ":", "char", ";"];
    unsafe {
        IN_BLOCK_STMNT = false;
        strict = true;
    }
    assert_eq!(stream, parse_declaration(&tok_vector));

    let doc = NO_STRICT;

    let tok_vector = vec![Token::new(String::from("int"), BASE_DATATYPE, PRIMITIVE_INT, 0, 0),
                          Token::new(String::from("a"), BASE_NONE, IDENTIFIER, 0, 0),
                          //   Token::new(String::from("="), BASE_NONE, OP_ASSIGN, 0, 0),
                          //   Token::new(String::from("5"), BASE_VALUE, NUM_INT, 0, 0),
                          Token::new(String::from(";"), BASE_NONE, SEMICOLON, 0, 0)];
    let stream = vec![doc.get_doc(), "static mut", "a", ":", "i32", ";"];
    unsafe {
        IN_BLOCK_STMNT = false;
        strict = false;
    }
    assert_eq!(stream, parse_declaration(&tok_vector));
    unsafe {
        strict = true;
    }
}

#[test]
fn test_parse_declaration() {
    let mut doc = STRICT;
    let mut tok_vector =
        vec![Token::new(String::from("float"), BASE_DATATYPE, PRIMITIVE_FLOAT, 0, 0),
             Token::new(String::from("a"), BASE_NONE, IDENTIFIER, 0, 0),
             Token::new(String::from(";"), BASE_NONE, SEMICOLON, 0, 0)];
    let mut stream = vec![doc.get_doc(), "let", "a", ":", "f32", ";"];
    unsafe {
        strict = true;
        IN_BLOCK_STMNT = true;
    }
    assert_eq!(stream, parse_declaration(&tok_vector));

    doc = NO_STRICT;
    tok_vector = vec![Token::new(String::from("short"), BASE_DATATYPE, PRIMITIVE_SHORT, 0, 0),
                      Token::new(String::from("a"), BASE_NONE, IDENTIFIER, 0, 0),
                      Token::new(String::from(";"), BASE_NONE, SEMICOLON, 0, 0)];
    stream = vec![doc.get_doc(), "let mut", "a", ":", "i16", ";"];
    unsafe {
        strict = false;
        IN_BLOCK_STMNT = true;
    }
    assert_eq!(stream, parse_declaration(&tok_vector));
    unsafe {
        strict = true;
        IN_BLOCK_STMNT = false;
    }
}

#[test]
fn test_parse_declaration_expr() {
    let mut doc = STRICT;

    unsafe {
        strict = true;
        IN_BLOCK_STMNT = false;
    }

    let mut tok_vector =
        vec![Token::new(String::from("float"), BASE_DATATYPE, PRIMITIVE_FLOAT, 0, 0),
             Token::new(String::from("a"), BASE_NONE, IDENTIFIER, 0, 0),
             Token::new(String::from("="), BASE_NONE, OP_ASSIGN, 0, 0),
             Token::new(String::from("2"), BASE_VALUE, NUM_INT, 0, 0),
             Token::new(String::from("+"), BASE_BINOP, OP_PLUS, 0, 0),
             Token::new(String::from("arr"), BASE_NONE, IDENTIFIER, 0, 0),
             Token::new(String::from("["), BASE_NONE, LEFT_SBRACKET, 0, 0),
             Token::new(String::from("5"), BASE_VALUE, NUM_INT, 0, 0),
             Token::new(String::from("]"), BASE_NONE, RIGHT_SBRACKET, 0, 0),
             Token::new(String::from("+"), BASE_BINOP, OP_PLUS, 0, 0),
             Token::new(String::from("3"), BASE_VALUE, NUM_INT, 0, 0),
             Token::new(String::from("-"), BASE_BINOP, OP_MINUS, 0, 0),
             Token::new(String::from("4"), BASE_VALUE, NUM_INT, 0, 0),
             Token::new(String::from("/"), BASE_BINOP, OP_DIV, 0, 0),
             Token::new(String::from("5"), BASE_VALUE, NUM_INT, 0, 0),
             Token::new(String::from("*"), BASE_BINOP, OP_MUL, 0, 0),
             Token::new(String::from("6.3"), BASE_VALUE, NUM_FLOAT, 0, 0),
             Token::new(String::from("%"), BASE_BINOP, OP_MOD, 0, 0),
             Token::new(String::from("7"), BASE_VALUE, NUM_INT, 0, 0),
             Token::new(String::from("+"), BASE_BINOP, OP_PLUS, 0, 0),
             Token::new(String::from("func"), BASE_NONE, IDENTIFIER, 0, 0),
             Token::new(String::from("("), BASE_NONE, LEFT_BRACKET, 0, 0),
             Token::new(String::from("5"), BASE_VALUE, NUM_INT, 0, 0),
             Token::new(String::from(","), BASE_NONE, COMMA, 0, 0),
             Token::new(String::from("arg"), BASE_NONE, IDENTIFIER, 0, 0),
             Token::new(String::from(")"), BASE_NONE, RIGHT_BRACKET, 0, 0),
             Token::new(String::from(";"), BASE_NONE, SEMICOLON, 0, 0)];
    let mut stream = vec![doc.get_doc(),
                          "static",
                          "a",
                          ":",
                          "f32",
                          "=",
                          "2+arr[5]+3-4/5*6.3%7+func(5,arg)",
                          ";"];

    assert_eq!(stream, parse_program(&tok_vector));

    doc = NO_STRICT;
    tok_vector = vec![Token::new(String::from("double"),
                                 BASE_DATATYPE,
                                 PRIMITIVE_DOUBLE,
                                 0,
                                 0),
                      Token::new(String::from("a"), BASE_NONE, IDENTIFIER, 0, 0),
                      Token::new(String::from("="), BASE_NONE, OP_ASSIGN, 0, 0),
                      Token::new(String::from("2"), BASE_VALUE, NUM_INT, 0, 0),
                      Token::new(String::from("+"), BASE_BINOP, OP_PLUS, 0, 0),
                      Token::new(String::from("arr"), BASE_NONE, IDENTIFIER, 0, 0),
                      Token::new(String::from("["), BASE_NONE, LEFT_SBRACKET, 0, 0),
                      Token::new(String::from("5"), BASE_VALUE, NUM_INT, 0, 0),
                      Token::new(String::from("]"), BASE_NONE, RIGHT_SBRACKET, 0, 0),
                      Token::new(String::from("+"), BASE_BINOP, OP_PLUS, 0, 0),
                      Token::new(String::from("3"), BASE_VALUE, NUM_INT, 0, 0),
                      Token::new(String::from("-"), BASE_BINOP, OP_MINUS, 0, 0),
                      Token::new(String::from("4"), BASE_VALUE, NUM_INT, 0, 0),
                      Token::new(String::from("/"), BASE_BINOP, OP_DIV, 0, 0),
                      Token::new(String::from("5"), BASE_VALUE, NUM_INT, 0, 0),
                      Token::new(String::from("*"), BASE_BINOP, OP_MUL, 0, 0),
                      Token::new(String::from("6.3"), BASE_VALUE, NUM_FLOAT, 0, 0),
                      Token::new(String::from("%"), BASE_BINOP, OP_MOD, 0, 0),
                      Token::new(String::from("7"), BASE_VALUE, NUM_INT, 0, 0),
                      Token::new(String::from("+"), BASE_BINOP, OP_PLUS, 0, 0),
                      Token::new(String::from("func"), BASE_NONE, IDENTIFIER, 0, 0),
                      Token::new(String::from("("), BASE_NONE, LEFT_BRACKET, 0, 0),
                      Token::new(String::from("5"), BASE_VALUE, NUM_INT, 0, 0),
                      Token::new(String::from(","), BASE_NONE, COMMA, 0, 0),
                      Token::new(String::from("arg"), BASE_NONE, IDENTIFIER, 0, 0),
                      Token::new(String::from(")"), BASE_NONE, RIGHT_BRACKET, 0, 0),
                      Token::new(String::from(";"), BASE_NONE, SEMICOLON, 0, 0)];
    stream = vec![doc.get_doc(),
                  "static mut",
                  "a",
                  ":",
                  "f64",
                  "=",
                  "2+arr[5]+3-4/5*6.3%7+func(5,arg)",
                  ";"];
    unsafe {
        strict = false;
        IN_BLOCK_STMNT = false;
    }
    assert_eq!(stream, parse_program(&tok_vector));
    unsafe {
        strict = true;
    }
}

#[test]
fn test_parse_array_declaration() {
    let mut doc = STRICT;
    unsafe {
        IN_BLOCK_STMNT = true;
        strict = true;
    }
    let mut tok_vector = vec![Token::new(String::from("int"), BASE_DATATYPE, PRIMITIVE_INT, 0, 0),
                              Token::new(String::from("a"), BASE_NONE, IDENTIFIER, 0, 0),
                              Token::new(String::from("["), BASE_NONE, LEFT_SBRACKET, 0, 0),
                              Token::new(String::from("5"), BASE_VALUE, NUM_INT, 0, 0),
                              Token::new(String::from("]"), BASE_NONE, RIGHT_SBRACKET, 0, 0),
                              Token::new(String::from(";"), BASE_NONE, SEMICOLON, 0, 0)];
    let mut stream = vec![doc.get_doc(), "let", "a", ":", "[i32;5]", ";"];

    assert_eq!(stream, parse_array_declaration(&tok_vector));
    unsafe {
        IN_BLOCK_STMNT = false;
    }

    doc = NO_STRICT;
    tok_vector = vec![Token::new(String::from("int"), BASE_DATATYPE, PRIMITIVE_INT, 0, 0),
                      Token::new(String::from("a"), BASE_NONE, IDENTIFIER, 0, 0),
                      Token::new(String::from("["), BASE_NONE, LEFT_SBRACKET, 0, 0),
                      Token::new(String::from("5"), BASE_VALUE, NUM_INT, 0, 0),
                      Token::new(String::from("]"), BASE_NONE, RIGHT_SBRACKET, 0, 0),
                      Token::new(String::from(";"), BASE_NONE, SEMICOLON, 0, 0)];
    stream = vec![doc.get_doc(), "let mut", "a", ":", "[i32;5]", ";"];
    unsafe {
        IN_BLOCK_STMNT = true;
        strict = false;
    }
    assert_eq!(stream, parse_array_declaration(&tok_vector));
    unsafe {
        IN_BLOCK_STMNT = false;
        strict = true;
    }
}

#[test]
fn test_parse_array_declaration_assignment() {
    let mut doc = STRICT;

    unsafe {
        IN_BLOCK_STMNT = true;
        strict = true;
    }
    let mut tok_vector =
        vec![Token::new(String::from("char"), BASE_DATATYPE, PRIMITIVE_CHAR, 0, 0),
             Token::new(String::from("a"), BASE_NONE, IDENTIFIER, 0, 0),
             Token::new(String::from("["), BASE_NONE, LEFT_SBRACKET, 0, 0),
             Token::new(String::from("5"), BASE_VALUE, NUM_INT, 0, 0),
             Token::new(String::from("]"), BASE_NONE, RIGHT_SBRACKET, 0, 0),
             Token::new(String::from("="), BASE_NONE, OP_ASSIGN, 0, 0),
             Token::new(String::from("{"), BASE_NONE, LEFT_CBRACE, 0, 0),
             Token::new(String::from("'a'"), BASE_VALUE, CHAR_VAL, 0, 0),
             Token::new(String::from(","), BASE_NONE, COMMA, 0, 0),
             Token::new(String::from("'e'"), BASE_VALUE, CHAR_VAL, 0, 0),
             Token::new(String::from(","), BASE_NONE, COMMA, 0, 0),
             Token::new(String::from("'i'"), BASE_VALUE, CHAR_VAL, 0, 0),
             Token::new(String::from(","), BASE_NONE, COMMA, 0, 0),
             Token::new(String::from("'o'"), BASE_VALUE, CHAR_VAL, 0, 0),
             Token::new(String::from(","), BASE_NONE, COMMA, 0, 0),
             Token::new(String::from("'u'"), BASE_VALUE, CHAR_VAL, 0, 0),
             Token::new(String::from("}"), BASE_NONE, RIGHT_CBRACE, 0, 0),
             Token::new(String::from(";"), BASE_NONE, SEMICOLON, 0, 0)];
    let mut stream = vec![doc.get_doc(),
                          "let",
                          "a",
                          ":",
                          "[char;5]",
                          "=",
                          "[",
                          "'a'",
                          ",",
                          "'e'",
                          ",",
                          "'i'",
                          ",",
                          "'o'",
                          ",",
                          "'u'",
                          "]",
                          ";"];

    assert_eq!(stream, parse_array_declaration(&tok_vector));
    unsafe {
        IN_BLOCK_STMNT = false;
        strict = false;
    }

    doc = NO_STRICT;
    tok_vector = vec![Token::new(String::from("char"), BASE_DATATYPE, PRIMITIVE_CHAR, 0, 0),
                      Token::new(String::from("a"), BASE_NONE, IDENTIFIER, 0, 0),
                      Token::new(String::from("["), BASE_NONE, LEFT_SBRACKET, 0, 0),
                      Token::new(String::from("5"), BASE_VALUE, NUM_INT, 0, 0),
                      Token::new(String::from("]"), BASE_NONE, RIGHT_SBRACKET, 0, 0),
                      Token::new(String::from("="), BASE_NONE, OP_ASSIGN, 0, 0),
                      Token::new(String::from("{"), BASE_NONE, LEFT_CBRACE, 0, 0),
                      Token::new(String::from("'a'"), BASE_VALUE, CHAR_VAL, 0, 0),
                      Token::new(String::from(","), BASE_NONE, COMMA, 0, 0),
                      Token::new(String::from("'e'"), BASE_VALUE, CHAR_VAL, 0, 0),
                      Token::new(String::from(","), BASE_NONE, COMMA, 0, 0),
                      Token::new(String::from("'i'"), BASE_VALUE, CHAR_VAL, 0, 0),
                      Token::new(String::from(","), BASE_NONE, COMMA, 0, 0),
                      Token::new(String::from("'o'"), BASE_VALUE, CHAR_VAL, 0, 0),
                      Token::new(String::from(","), BASE_NONE, COMMA, 0, 0),
                      Token::new(String::from("'u'"), BASE_VALUE, CHAR_VAL, 0, 0),
                      Token::new(String::from("}"), BASE_NONE, RIGHT_CBRACE, 0, 0),
                      Token::new(String::from(";"), BASE_NONE, SEMICOLON, 0, 0)];
    stream = vec![doc.get_doc(),
                  "let mut",
                  "a",
                  ":",
                  "[char;5]",
                  "=",
                  "[",
                  "'a'",
                  ",",
                  "'e'",
                  ",",
                  "'i'",
                  ",",
                  "'o'",
                  ",",
                  "'u'",
                  "]",
                  ";"];
    unsafe {
        IN_BLOCK_STMNT = true;
        strict = false;
    }
    assert_eq!(stream, parse_array_declaration(&tok_vector));
    unsafe {
        IN_BLOCK_STMNT = false;
        strict = true;
    }
}

#[test]
fn test_parse_declaration_assignment() {
    let mut doc = STRICT;
    unsafe {
        IN_BLOCK_STMNT = true;
        strict = true;
    }
    let mut tok_vector = vec![Token::new(String::from("int"), BASE_DATATYPE, PRIMITIVE_INT, 0, 0),
                              Token::new(String::from("a"), BASE_NONE, IDENTIFIER, 0, 0),
                              Token::new(String::from("="), BASE_NONE, OP_ASSIGN, 0, 0),
                              Token::new(String::from("5"), BASE_VALUE, NUM_INT, 0, 0),
                              Token::new(String::from(";"), BASE_NONE, SEMICOLON, 0, 0)];
    let mut stream = vec![doc.get_doc(), "let", "a", ":", "i32", "=", "5", ";"];

    assert_eq!(stream, parse_declaration(&tok_vector));


    doc = NO_STRICT;
    tok_vector = vec![Token::new(String::from("int"), BASE_DATATYPE, PRIMITIVE_INT, 0, 0),
                      Token::new(String::from("a"), BASE_NONE, IDENTIFIER, 0, 0),
                      Token::new(String::from("="), BASE_NONE, OP_ASSIGN, 0, 0),
                      Token::new(String::from("5"), BASE_VALUE, NUM_INT, 0, 0),
                      Token::new(String::from(";"), BASE_NONE, SEMICOLON, 0, 0)];
    stream = vec![doc.get_doc(), "let mut", "a", ":", "i32", "=", "5", ";"];
    unsafe {
        strict = false;
        IN_BLOCK_STMNT = true;
    }
    assert_eq!(stream, parse_declaration(&tok_vector));
    unsafe {
        strict = true;
        IN_BLOCK_STMNT = false;
    }
}

// variable assignment
#[test]
fn test_parse_assignment_single() {
    let mut tok_vector = vec![Token::new(String::from("a"), BASE_NONE, IDENTIFIER, 0, 0),
                              Token::new(String::from("="), BASE_NONE, OP_ASSIGN, 0, 0),
                              Token::new(String::from("5"), BASE_VALUE, NUM_INT, 0, 0),
                              Token::new(String::from(";"), BASE_NONE, SEMICOLON, 0, 0)];
    let mut stream = vec!["a", "=", "5", ";"];
    assert_eq!(stream, parse_assignment(&tok_vector));

    tok_vector = vec![Token::new(String::from("a"), BASE_NONE, IDENTIFIER, 0, 0),
                      Token::new(String::from("="), BASE_NONE, OP_ASSIGN, 0, 0),
                      Token::new(String::from("b"), BASE_NONE, IDENTIFIER, 0, 0),
                      Token::new(String::from(";"), BASE_NONE, SEMICOLON, 0, 0)];
    stream = vec!["a", "=", "b", ";"];
    assert_eq!(stream, parse_assignment(&tok_vector));
}

#[test]
fn test_parse_assignment_func_val() {
    let tok_vector = vec![Token::new(String::from("a"), BASE_NONE, IDENTIFIER, 0, 0),
                          Token::new(String::from("="), BASE_NONE, OP_ASSIGN, 0, 0),
                          Token::new(String::from("func"), BASE_NONE, IDENTIFIER, 0, 0),
                          Token::new(String::from("("), BASE_NONE, LEFT_BRACKET, 0, 0),
                          Token::new(String::from(")"), BASE_NONE, RIGHT_BRACKET, 0, 0),
                          Token::new(String::from(";"), BASE_NONE, SEMICOLON, 0, 0)];
    let stream = vec!["a", "=", "func", "(", ")", ";"];
    assert_eq!(stream, parse_assignment(&tok_vector));
}

#[test]
fn test_parse_assignment_arr_val_binop() {
    let tok_vector = vec![Token::new(String::from("a"), BASE_NONE, IDENTIFIER, 0, 0),
                          Token::new(String::from("="), BASE_NONE, OP_ASSIGN, 0, 0),
                          Token::new(String::from("arr1"), BASE_NONE, IDENTIFIER, 0, 0),
                          Token::new(String::from("["), BASE_NONE, LEFT_SBRACKET, 0, 0),
                          Token::new(String::from("4"), BASE_VALUE, NUM_INT, 0, 0),
                          Token::new(String::from("]"), BASE_NONE, RIGHT_SBRACKET, 0, 0),
                          Token::new(String::from("+"), BASE_BINOP, OP_PLUS, 0, 0),
                          Token::new(String::from("34"), BASE_VALUE, NUM_INT, 0, 0),
                          Token::new(String::from("*"), BASE_BINOP, OP_MUL, 0, 0),
                          Token::new(String::from("b"), BASE_NONE, IDENTIFIER, 0, 0),
                          Token::new(String::from("/"), BASE_BINOP, OP_DIV, 0, 0),
                          Token::new(String::from("func2"), BASE_NONE, IDENTIFIER, 0, 0),
                          Token::new(String::from("("), BASE_NONE, LEFT_BRACKET, 0, 0),
                          Token::new(String::from("34"), BASE_VALUE, NUM_INT, 0, 0),
                          Token::new(String::from(","), BASE_NONE, COMMA, 0, 0),
                          Token::new(String::from("c"), BASE_NONE, IDENTIFIER, 0, 0),
                          Token::new(String::from(")"), BASE_NONE, RIGHT_BRACKET, 0, 0),
                          Token::new(String::from(";"), BASE_NONE, SEMICOLON, 0, 0)];
    let stream = vec!["a", "=", "arr1", "[", "4", "]", "+", "34", "*", "b", "/", "func2", "(",
                      "34", ",", "c", ")", ";"];
    assert_eq!(stream, parse_assignment(&tok_vector));
}

#[test]
fn test_parse_assignment_array_val() {
    let tok_vector = vec![Token::new(String::from("a"), BASE_NONE, IDENTIFIER, 0, 0),
                          Token::new(String::from("="), BASE_NONE, OP_ASSIGN, 0, 0),
                          Token::new(String::from("arr"), BASE_NONE, IDENTIFIER, 0, 0),
                          Token::new(String::from("["), BASE_NONE, LEFT_SBRACKET, 0, 0),
                          Token::new(String::from("3"), BASE_VALUE, NUM_INT, 0, 0),
                          Token::new(String::from("]"), BASE_NONE, RIGHT_SBRACKET, 0, 0),
                          Token::new(String::from(";"), BASE_NONE, SEMICOLON, 0, 0)];
    let stream = vec!["a", "=", "arr", "[", "3", "]", ";"];
    assert_eq!(stream, parse_assignment(&tok_vector));
}

#[test]
fn test_parse_assignment_val_to_array() {
    let tok_vector = vec![Token::new(String::from("a"), BASE_NONE, IDENTIFIER, 0, 0),
                          Token::new(String::from("["), BASE_NONE, LEFT_SBRACKET, 0, 0),
                          Token::new(String::from("3"), BASE_VALUE, NUM_INT, 0, 0),
                          Token::new(String::from("]"), BASE_NONE, RIGHT_SBRACKET, 0, 0),
                          Token::new(String::from("="), BASE_NONE, OP_ASSIGN, 0, 0),
                          Token::new(String::from("'d'"), BASE_VALUE, CHAR_VAL, 0, 0),
                          Token::new(String::from(";"), BASE_NONE, SEMICOLON, 0, 0)];
    let stream = vec!["a", "[", "3", "]", "=", "'d'", ";"];
    assert_eq!(stream, parse_program(&tok_vector));
}

#[test]
fn test_parse_assignment_func_val_binop() {
    let tok_vector = vec![Token::new(String::from("a"), BASE_NONE, IDENTIFIER, 0, 0),
                          Token::new(String::from("="), BASE_NONE, OP_ASSIGN, 0, 0),
                          Token::new(String::from("func1"), BASE_NONE, IDENTIFIER, 0, 0),
                          Token::new(String::from("("), BASE_NONE, LEFT_BRACKET, 0, 0),
                          Token::new(String::from(")"), BASE_NONE, RIGHT_BRACKET, 0, 0),
                          Token::new(String::from("+"), BASE_BINOP, OP_PLUS, 0, 0),
                          Token::new(String::from("34"), BASE_VALUE, NUM_INT, 0, 0),
                          Token::new(String::from("*"), BASE_BINOP, OP_MUL, 0, 0),
                          Token::new(String::from("b"), BASE_NONE, IDENTIFIER, 0, 0),
                          Token::new(String::from("/"), BASE_BINOP, OP_DIV, 0, 0),
                          Token::new(String::from("func2"), BASE_NONE, IDENTIFIER, 0, 0),
                          Token::new(String::from("("), BASE_NONE, LEFT_BRACKET, 0, 0),
                          Token::new(String::from("34"), BASE_VALUE, NUM_INT, 0, 0),
                          Token::new(String::from(","), BASE_NONE, COMMA, 0, 0),
                          Token::new(String::from("c"), BASE_NONE, IDENTIFIER, 0, 0),
                          Token::new(String::from(")"), BASE_NONE, RIGHT_BRACKET, 0, 0),
                          Token::new(String::from(";"), BASE_NONE, SEMICOLON, 0, 0)];
    let stream = vec!["a", "=", "func1", "(", ")", "+", "34", "*", "b", "/", "func2", "(", "34",
                      ",", "c", ")", ";"];
    assert_eq!(stream, parse_assignment(&tok_vector));
}

#[test]
fn test_parse_assignment_multiple() {
    let tok_vector = vec![Token::new(String::from("a"), BASE_NONE, IDENTIFIER, 0, 0),
                          Token::new(String::from("="), BASE_NONE, OP_ASSIGN, 0, 0),
                          Token::new(String::from("b"), BASE_NONE, IDENTIFIER, 0, 0),
                          Token::new(String::from("="), BASE_NONE, OP_ASSIGN, 0, 0),
                          Token::new(String::from("c"), BASE_NONE, IDENTIFIER, 0, 0),
                          Token::new(String::from("="), BASE_NONE, OP_ASSIGN, 0, 0),
                          Token::new(String::from("d"), BASE_NONE, IDENTIFIER, 0, 0),
                          Token::new(String::from("="), BASE_NONE, OP_ASSIGN, 0, 0),
                          Token::new(String::from("5"), BASE_VALUE, NUM_INT, 0, 0),
                          Token::new(String::from(";"), BASE_NONE, SEMICOLON, 0, 0)];
    let stream = vec!["d", "=", "5", ";", "c", "=", "d", ";", "b", "=", "c", ";", "a", "=", "b",
                      ";"];
    assert_eq!(stream, parse_assignment(&tok_vector));
}

#[test]
fn test_parse_assignment_commas() {
    let tok_vector = vec![Token::new(String::from("a"), BASE_NONE, IDENTIFIER, 0, 0),
                          Token::new(String::from("="), BASE_NONE, OP_ASSIGN, 0, 0),
                          Token::new(String::from("1"), BASE_VALUE, NUM_INT, 0, 0),
                          Token::new(String::from(","), BASE_NONE, COMMA, 0, 0),
                          Token::new(String::from("b"), BASE_NONE, IDENTIFIER, 0, 0),
                          Token::new(String::from("="), BASE_NONE, OP_ASSIGN, 0, 0),
                          Token::new(String::from("2"), BASE_VALUE, NUM_INT, 0, 0),
                          Token::new(String::from(","), BASE_NONE, COMMA, 0, 0),
                          Token::new(String::from("c"), BASE_NONE, IDENTIFIER, 0, 0),
                          Token::new(String::from("="), BASE_NONE, OP_ASSIGN, 0, 0),
                          Token::new(String::from("3"), BASE_VALUE, NUM_INT, 0, 0),
                          Token::new(String::from(";"), BASE_NONE, SEMICOLON, 0, 0)];
    let stream = vec!["a", "=", "1", ";", "b", "=", "2", ";", "c", "=", "3", ";"];
    assert_eq!(stream, parse_program(&tok_vector));
}

#[test]
fn test_parse_assignment_binops() {
    let tok_vector = vec![Token::new(String::from("a"), BASE_NONE, IDENTIFIER, 0, 0),
                          Token::new(String::from("="), BASE_NONE, OP_ASSIGN, 0, 0),
                          Token::new(String::from("2"), BASE_VALUE, NUM_INT, 0, 0),
                          Token::new(String::from("+"), BASE_BINOP, OP_PLUS, 0, 0),
                          Token::new(String::from("3"), BASE_VALUE, NUM_INT, 0, 0),
                          Token::new(String::from("-"), BASE_BINOP, OP_MINUS, 0, 0),
                          Token::new(String::from("4"), BASE_VALUE, NUM_INT, 0, 0),
                          Token::new(String::from("/"), BASE_BINOP, OP_DIV, 0, 0),
                          Token::new(String::from("5"), BASE_VALUE, NUM_INT, 0, 0),
                          Token::new(String::from("*"), BASE_BINOP, OP_MUL, 0, 0),
                          Token::new(String::from("6.3"), BASE_VALUE, NUM_FLOAT, 0, 0),
                          Token::new(String::from("%"), BASE_BINOP, OP_MOD, 0, 0),
                          Token::new(String::from("7"), BASE_VALUE, NUM_INT, 0, 0),
                          Token::new(String::from(";"), BASE_NONE, SEMICOLON, 0, 0)];
    let stream = vec!["a", "=", "2", "+", "3", "-", "4", "/", "5", "*", "6.3", "%", "7", ";"];
    assert_eq!(stream, parse_assignment(&tok_vector));
}

#[test]
fn test_parse_assignment_preops() {
    let mut tok_vector = vec![Token::new(String::from("a"), BASE_NONE, IDENTIFIER, 0, 0),
                              Token::new(String::from("="), BASE_NONE, OP_ASSIGN, 0, 0),
                              Token::new(String::from("++"), BASE_UNOP, OP_INC, 0, 0),
                              Token::new(String::from("b"), BASE_NONE, IDENTIFIER, 0, 0),
                              Token::new(String::from(";"), BASE_NONE, SEMICOLON, 0, 0)];
    let mut stream = vec!["a", "=", "(", "b", "+=1", ")", ";"];
    assert_eq!(stream, parse_assignment(&tok_vector));

    tok_vector = vec![Token::new(String::from("a"), BASE_NONE, IDENTIFIER, 0, 0),
                      Token::new(String::from("="), BASE_NONE, OP_ASSIGN, 0, 0),
                      Token::new(String::from("--"), BASE_UNOP, OP_DEC, 0, 0),
                      Token::new(String::from("b"), BASE_NONE, IDENTIFIER, 0, 0),
                      Token::new(String::from(";"), BASE_NONE, SEMICOLON, 0, 0)];
    stream = vec!["a", "=", "(", "b", "-=1", ")", ";"];

    assert_eq!(stream, parse_assignment(&tok_vector));

    tok_vector = vec![Token::new(String::from("--"), BASE_UNOP, OP_DEC, 0, 0),
                      Token::new(String::from("b"), BASE_NONE, IDENTIFIER, 0, 0),
                      Token::new(String::from(";"), BASE_NONE, SEMICOLON, 0, 0)];
    stream = vec!["b", "-=1", ";"];

    assert_eq!(stream, parse_program(&tok_vector));
}

#[test]
fn test_parse_assignment_postops() {
    let mut tok_vector = vec![Token::new(String::from("a"), BASE_NONE, IDENTIFIER, 0, 0),
                              Token::new(String::from("="), BASE_NONE, OP_ASSIGN, 0, 0),
                              Token::new(String::from("b"), BASE_NONE, IDENTIFIER, 0, 0),
                              Token::new(String::from("++"), BASE_UNOP, OP_INC, 0, 0),
                              Token::new(String::from(";"), BASE_NONE, SEMICOLON, 0, 0)];
    let mut stream = vec!["a", "=", "b", ";", "b", "+=1", ";"];
    assert_eq!(stream, parse_assignment(&tok_vector));

    tok_vector = vec![Token::new(String::from("a"), BASE_NONE, IDENTIFIER, 0, 0),
                      Token::new(String::from("="), BASE_NONE, OP_ASSIGN, 0, 0),
                      Token::new(String::from("b"), BASE_NONE, IDENTIFIER, 0, 0),
                      Token::new(String::from("--"), BASE_UNOP, OP_DEC, 0, 0),
                      Token::new(String::from(";"), BASE_NONE, SEMICOLON, 0, 0)];
    stream = vec!["a", "=", "b", ";", "b", "-=1", ";"];
    assert_eq!(stream, parse_assignment(&tok_vector));

    tok_vector = vec![Token::new(String::from("b"), BASE_NONE, IDENTIFIER, 0, 0),
                      Token::new(String::from("--"), BASE_UNOP, OP_DEC, 0, 0),
                      Token::new(String::from(";"), BASE_NONE, SEMICOLON, 0, 0)];
    stream = vec!["b", "-=1", ";"];
    assert_eq!(stream, parse_program(&tok_vector));
}

#[test]
fn test_parse_assignment_pre_bin_ops() {
    let mut tok_vector = vec![Token::new(String::from("a"), BASE_NONE, IDENTIFIER, 0, 0),
                              Token::new(String::from("="), BASE_NONE, OP_ASSIGN, 0, 0),
                              Token::new(String::from("b"), BASE_NONE, IDENTIFIER, 0, 0),
                              Token::new(String::from("-"), BASE_BINOP, OP_MINUS, 0, 0),
                              Token::new(String::from("++"), BASE_UNOP, OP_INC, 0, 0),
                              Token::new(String::from("c"), BASE_NONE, IDENTIFIER, 0, 0),
                              Token::new(String::from(";"), BASE_NONE, SEMICOLON, 0, 0)];
    let mut stream = vec!["a", "=", "b", "-", "(", "c", "+=1", ")", ";"];
    assert_eq!(stream, parse_assignment(&tok_vector));

    tok_vector = vec![Token::new(String::from("a"), BASE_NONE, IDENTIFIER, 0, 0),
                      Token::new(String::from("="), BASE_NONE, OP_ASSIGN, 0, 0),
                      Token::new(String::from("b"), BASE_NONE, IDENTIFIER, 0, 0),
                      Token::new(String::from("+"), BASE_BINOP, OP_PLUS, 0, 0),
                      Token::new(String::from("--"), BASE_UNOP, OP_DEC, 0, 0),
                      Token::new(String::from("c"), BASE_NONE, IDENTIFIER, 0, 0),
                      Token::new(String::from(";"), BASE_NONE, SEMICOLON, 0, 0)];
    stream = vec!["a", "=", "b", "+", "(", "c", "-=1", ")", ";"];
    assert_eq!(stream, parse_assignment(&tok_vector));
}

#[test]
fn test_parse_assignment_post_bin_ops() {
    let mut tok_vector = vec![Token::new(String::from("a"), BASE_NONE, IDENTIFIER, 0, 0),
                              Token::new(String::from("="), BASE_NONE, OP_ASSIGN, 0, 0),
                              Token::new(String::from("b"), BASE_NONE, IDENTIFIER, 0, 0),
                              Token::new(String::from("--"), BASE_UNOP, OP_DEC, 0, 0),
                              Token::new(String::from("-"), BASE_BINOP, OP_MINUS, 0, 0),
                              Token::new(String::from("c"), BASE_NONE, IDENTIFIER, 0, 0),
                              Token::new(String::from(";"), BASE_NONE, SEMICOLON, 0, 0)];
    let mut stream = vec!["a", "=", "b", "-", "c", ";", "b", "-=1", ";"];
    assert_eq!(stream, parse_assignment(&tok_vector));

    tok_vector = vec![Token::new(String::from("a"), BASE_NONE, IDENTIFIER, 0, 0),
                      Token::new(String::from("="), BASE_NONE, OP_ASSIGN, 0, 0),
                      Token::new(String::from("b"), BASE_NONE, IDENTIFIER, 0, 0),
                      Token::new(String::from("++"), BASE_UNOP, OP_INC, 0, 0),
                      Token::new(String::from("+"), BASE_BINOP, OP_PLUS, 0, 0),
                      Token::new(String::from("c"), BASE_NONE, IDENTIFIER, 0, 0),
                      Token::new(String::from(";"), BASE_NONE, SEMICOLON, 0, 0)];
    stream = vec!["a", "=", "b", "+", "c", ";", "b", "+=1", ";"];
    assert_eq!(stream, parse_assignment(&tok_vector));
}

#[test]
fn test_parse_assignment_pre_post_ops() {
    let tok_vector = vec![Token::new(String::from("a"), BASE_NONE, IDENTIFIER, 0, 0),
                          Token::new(String::from("="), BASE_NONE, OP_ASSIGN, 0, 0),
                          Token::new(String::from("b"), BASE_NONE, IDENTIFIER, 0, 0),
                          Token::new(String::from("--"), BASE_UNOP, OP_DEC, 0, 0),
                          Token::new(String::from("-"), BASE_BINOP, OP_MINUS, 0, 0),
                          Token::new(String::from("++"), BASE_UNOP, OP_INC, 0, 0),
                          Token::new(String::from("c"), BASE_NONE, IDENTIFIER, 0, 0),
                          Token::new(String::from("+"), BASE_BINOP, OP_PLUS, 0, 0),
                          Token::new(String::from("--"), BASE_UNOP, OP_DEC, 0, 0),
                          Token::new(String::from("d"), BASE_NONE, IDENTIFIER, 0, 0),
                          Token::new(String::from("-"), BASE_BINOP, OP_MINUS, 0, 0),
                          Token::new(String::from("e"), BASE_NONE, IDENTIFIER, 0, 0),
                          Token::new(String::from("++"), BASE_UNOP, OP_INC, 0, 0),
                          Token::new(String::from(";"), BASE_NONE, SEMICOLON, 0, 0)];
    let stream = vec!["a", "=", "b", "-", "(", "c", "+=1", ")", "+", "(", "d", "-=1", ")", "-",
                      "e", ";", "b", "-=1", ";", "e", "+=1", ";"];
    assert_eq!(stream, parse_assignment(&tok_vector));


}

// function
#[test]
fn test_parse_function_skip_decl() {
    unsafe {
        IN_BLOCK_STMNT = true;
    }
    let tok_vector = vec![Token::new(String::from("int"), BASE_DATATYPE, PRIMITIVE_INT, 0, 0),
                          Token::new(String::from("a"), BASE_NONE, IDENTIFIER, 0, 0),
                          Token::new(String::from("("), BASE_NONE, LEFT_BRACKET, 0, 0),
                          Token::new(String::from("int"), BASE_DATATYPE, PRIMITIVE_INT, 0, 0),
                          Token::new(String::from("a1"), BASE_NONE, IDENTIFIER, 0, 0),
                          Token::new(String::from(","), BASE_NONE, COMMA, 0, 0),
                          Token::new(String::from("char"), BASE_DATATYPE, PRIMITIVE_CHAR, 0, 0),
                          Token::new(String::from("a2"), BASE_NONE, IDENTIFIER, 0, 0),
                          Token::new(String::from(")"), BASE_NONE, RIGHT_BRACKET, 0, 0),
                          Token::new(String::from(";"), BASE_NONE, SEMICOLON, 0, 0)];
    let stream: Vec<String> = Vec::new();
    assert_eq!(stream, parse_program(&tok_vector));
    unsafe {
        IN_BLOCK_STMNT = false;
    }
}

#[test]
fn test_parse_main() {
    let mut doc = STRICT;
    let mut tok_vector =
        vec![Token::new(String::from("int"), BASE_DATATYPE, PRIMITIVE_INT, 0, 0),
             Token::new(String::from("main"), BASE_NONE, MAIN, 0, 0),
             Token::new(String::from("("), BASE_NONE, LEFT_BRACKET, 0, 0),
             Token::new(String::from("int"), BASE_DATATYPE, PRIMITIVE_INT, 0, 0),
             Token::new(String::from("a1"), BASE_NONE, IDENTIFIER, 0, 0),
             Token::new(String::from(","), BASE_NONE, COMMA, 0, 0),
             Token::new(String::from("char"), BASE_DATATYPE, PRIMITIVE_CHAR, 0, 0),
             Token::new(String::from("a2"), BASE_NONE, IDENTIFIER, 0, 0),
             Token::new(String::from(")"), BASE_NONE, RIGHT_BRACKET, 0, 0),
             Token::new(String::from("{"), BASE_NONE, LEFT_CBRACE, 0, 0),
             Token::new(String::from("a1"), BASE_NONE, IDENTIFIER, 0, 0),
             Token::new(String::from("="), BASE_NONE, OP_ASSIGN, 0, 0),
             Token::new(String::from("1"), BASE_VALUE, NUM_INT, 0, 0),
             Token::new(String::from(";"), BASE_NONE, SEMICOLON, 0, 0),
             Token::new(String::from("a2"), BASE_NONE, IDENTIFIER, 0, 0),
             Token::new(String::from("="), BASE_NONE, OP_ASSIGN, 0, 0),
             Token::new(String::from("'a'"), BASE_VALUE, CHAR_VAL, 0, 0),
             Token::new(String::from(";"), BASE_NONE, SEMICOLON, 0, 0),
             Token::new(String::from("}"), BASE_NONE, RIGHT_CBRACE, 0, 0)];
    let mut stream = vec!["fn",
                          "main",
                          "(",
                          ")",
                          "{",
                          doc.get_doc(),
                          "let argv: Vec<_> = std::env::args().collect();",
                          "let argc = argv.len();",
                          "a1",
                          "=",
                          "1",
                          ";",
                          "a2",
                          "=",
                          "'a'",
                          ";",
                          "}"];

    unsafe {
        IN_BLOCK_STMNT = true;
        strict = true;
    }
    assert_eq!(stream, parse_function(&tok_vector));
    unsafe {
        IN_BLOCK_STMNT = false;
    }

    doc = NO_STRICT;
    tok_vector = vec![Token::new(String::from("int"), BASE_DATATYPE, PRIMITIVE_INT, 0, 0),
                      Token::new(String::from("main"), BASE_NONE, MAIN, 0, 0),
                      Token::new(String::from("("), BASE_NONE, LEFT_BRACKET, 0, 0),
                      Token::new(String::from("int"), BASE_DATATYPE, PRIMITIVE_INT, 0, 0),
                      Token::new(String::from("a1"), BASE_NONE, IDENTIFIER, 0, 0),
                      Token::new(String::from(","), BASE_NONE, COMMA, 0, 0),
                      Token::new(String::from("char"), BASE_DATATYPE, PRIMITIVE_CHAR, 0, 0),
                      Token::new(String::from("a2"), BASE_NONE, IDENTIFIER, 0, 0),
                      Token::new(String::from(")"), BASE_NONE, RIGHT_BRACKET, 0, 0),
                      Token::new(String::from("{"), BASE_NONE, LEFT_CBRACE, 0, 0),
                      Token::new(String::from("a1"), BASE_NONE, IDENTIFIER, 0, 0),
                      Token::new(String::from("="), BASE_NONE, OP_ASSIGN, 0, 0),
                      Token::new(String::from("1"), BASE_VALUE, NUM_INT, 0, 0),
                      Token::new(String::from(";"), BASE_NONE, SEMICOLON, 0, 0),
                      Token::new(String::from("a2"), BASE_NONE, IDENTIFIER, 0, 0),
                      Token::new(String::from("="), BASE_NONE, OP_ASSIGN, 0, 0),
                      Token::new(String::from("'a'"), BASE_VALUE, CHAR_VAL, 0, 0),
                      Token::new(String::from(";"), BASE_NONE, SEMICOLON, 0, 0),
                      Token::new(String::from("}"), BASE_NONE, RIGHT_CBRACE, 0, 0)];
    stream = vec!["fn",
                  "main",
                  "(",
                  ")",
                  "{",
                  doc.get_doc(),
                  "let mut argv: Vec<_> = std::env::args().collect();",
                  "let mut argc = argv.len();",
                  "a1",
                  "=",
                  "1",
                  ";",
                  "a2",
                  "=",
                  "'a'",
                  ";",
                  "}"];

    unsafe {
        IN_BLOCK_STMNT = true;
        strict = false;
    }
    assert_eq!(stream, parse_function(&tok_vector));
    unsafe {
        IN_BLOCK_STMNT = false;
        strict = true;
    }
}

#[test]
fn test_parse_function() {
    unsafe {
        IN_BLOCK_STMNT = true;
    }
    let tok_vector = vec![Token::new(String::from("int"), BASE_DATATYPE, PRIMITIVE_INT, 0, 0),
                          Token::new(String::from("a"), BASE_NONE, IDENTIFIER, 0, 0),
                          Token::new(String::from("("), BASE_NONE, LEFT_BRACKET, 0, 0),
                          Token::new(String::from("int"), BASE_DATATYPE, PRIMITIVE_INT, 0, 0),
                          Token::new(String::from("a1"), BASE_NONE, IDENTIFIER, 0, 0),
                          Token::new(String::from(","), BASE_NONE, COMMA, 0, 0),
                          Token::new(String::from("char"), BASE_DATATYPE, PRIMITIVE_CHAR, 0, 0),
                          Token::new(String::from("a2"), BASE_NONE, IDENTIFIER, 0, 0),
                          Token::new(String::from(")"), BASE_NONE, RIGHT_BRACKET, 0, 0),
                          Token::new(String::from("{"), BASE_NONE, LEFT_CBRACE, 0, 0),
                          Token::new(String::from("a1"), BASE_NONE, IDENTIFIER, 0, 0),
                          Token::new(String::from("="), BASE_NONE, OP_ASSIGN, 0, 0),
                          Token::new(String::from("1"), BASE_VALUE, NUM_INT, 0, 0),
                          Token::new(String::from(";"), BASE_NONE, SEMICOLON, 0, 0),
                          Token::new(String::from("a2"), BASE_NONE, IDENTIFIER, 0, 0),
                          Token::new(String::from("="), BASE_NONE, OP_ASSIGN, 0, 0),
                          Token::new(String::from("'a'"), BASE_VALUE, CHAR_VAL, 0, 0),
                          Token::new(String::from(";"), BASE_NONE, SEMICOLON, 0, 0),
                          Token::new(String::from("}"), BASE_NONE, RIGHT_CBRACE, 0, 0)];
    let stream = vec!["fn", "a", "(", "a1", ":", "i32", ",", "a2", ":", "char", ")", "->", "i32",
                      "{", "a1", "=", "1", ";", "a2", "=", "'a'", ";", "}"];
    assert_eq!(stream, parse_function(&tok_vector));
    unsafe {
        IN_BLOCK_STMNT = false;
    }
}

#[test]
fn test_parse_function_no_args() {
    unsafe {
        IN_BLOCK_STMNT = true;
    }
    let doc = NO_RETURN;
    let tok_vector = vec![Token::new(String::from("int"), BASE_DATATYPE, PRIMITIVE_INT, 0, 0),
                          Token::new(String::from("a"), BASE_NONE, IDENTIFIER, 0, 0),
                          Token::new(String::from("("), BASE_NONE, LEFT_BRACKET, 0, 0),
                          Token::new(String::from(")"), BASE_NONE, RIGHT_BRACKET, 0, 0),
                          Token::new(String::from("{"), BASE_NONE, LEFT_CBRACE, 0, 0),
                          Token::new(String::from("a"), BASE_NONE, IDENTIFIER, 0, 0),
                          Token::new(String::from("="), BASE_NONE, OP_ASSIGN, 0, 0),
                          Token::new(String::from("1"), BASE_VALUE, NUM_INT, 0, 0),
                          Token::new(String::from(";"), BASE_NONE, SEMICOLON, 0, 0),
                          Token::new(String::from("return"), BASE_NONE, KEYWORD_RETURN, 0, 0),
                          Token::new(String::from("a"), BASE_NONE, IDENTIFIER, 0, 0),
                          Token::new(String::from(";"), BASE_NONE, SEMICOLON, 0, 0),
                          Token::new(String::from("}"), BASE_NONE, RIGHT_CBRACE, 0, 0)];
    let stream =
        vec!["fn", "a", "(", ")", "->", "i32", "{", "a", "=", "1", ";", doc.get_doc(), "a", "}"];
    assert_eq!(stream, parse_program(&tok_vector));
    unsafe {
        IN_BLOCK_STMNT = false;
    }
}

#[test]
fn test_parse_function_no_ret() {
    unsafe {
        IN_BLOCK_STMNT = true;
    }
    let tok_vector = vec![Token::new(String::from("void"), BASE_DATATYPE, PRIMITIVE_VOID, 0, 0),
                          Token::new(String::from("a"), BASE_NONE, IDENTIFIER, 0, 0),
                          Token::new(String::from("("), BASE_NONE, LEFT_BRACKET, 0, 0),
                          Token::new(String::from("int"), BASE_DATATYPE, PRIMITIVE_INT, 0, 0),
                          Token::new(String::from("a1"), BASE_NONE, IDENTIFIER, 0, 0),
                          Token::new(String::from(")"), BASE_NONE, RIGHT_BRACKET, 0, 0),
                          Token::new(String::from("{"), BASE_NONE, LEFT_CBRACE, 0, 0),
                          Token::new(String::from("a1"), BASE_NONE, IDENTIFIER, 0, 0),
                          Token::new(String::from("="), BASE_NONE, OP_ASSIGN, 0, 0),
                          Token::new(String::from("1"), BASE_VALUE, NUM_INT, 0, 0),
                          Token::new(String::from(";"), BASE_NONE, SEMICOLON, 0, 0),
                          Token::new(String::from("}"), BASE_NONE, RIGHT_CBRACE, 0, 0)];
    let stream = vec!["fn", "a", "(", "a1", ":", "i32", ")", "{", "a1", "=", "1", ";", "}"];
    assert_eq!(stream, parse_function(&tok_vector));
    unsafe {
        IN_BLOCK_STMNT = false;
    }
}

// while loop
#[test]
fn test_parse_while_braces() {
    let tok_vector = vec![Token::new(String::from("while"), BASE_NONE, KEYWORD_WHILE, 0, 0),
                          Token::new(String::from("("), BASE_NONE, LEFT_BRACKET, 0, 0),
                          Token::new(String::from("a"), BASE_NONE, IDENTIFIER, 0, 0),
                          Token::new(String::from("=="), BASE_BINOP, OP_EQU, 0, 0),
                          Token::new(String::from("a"), BASE_NONE, IDENTIFIER, 0, 0),
                          Token::new(String::from(")"), BASE_NONE, RIGHT_BRACKET, 0, 0),
                          Token::new(String::from("{"), BASE_NONE, LEFT_CBRACE, 0, 0),
                          Token::new(String::from("/*Do something here*/"),
                                     BASE_COMMENT,
                                     COMMENT_MULTI,
                                     0,
                                     0),
                          Token::new(String::from("}"), BASE_NONE, RIGHT_CBRACE, 0, 0)];
    let stream =
        vec!["while", "(", "a", "==", "a", ")", "== true", "{", "/*Do something here*/\n", "}"];

    assert_eq!(stream, parse_while(&tok_vector));
}

#[test]
fn test_parse_while_no_braces() {
    let tok_vector = vec![Token::new(String::from("while"), BASE_NONE, KEYWORD_WHILE, 0, 0),
                          Token::new(String::from("("), BASE_NONE, LEFT_BRACKET, 0, 0),
                          Token::new(String::from("a"), BASE_NONE, IDENTIFIER, 0, 0),
                          Token::new(String::from("=="), BASE_BINOP, OP_EQU, 0, 0),
                          Token::new(String::from("a"), BASE_NONE, IDENTIFIER, 0, 0),
                          Token::new(String::from(")"), BASE_NONE, RIGHT_BRACKET, 0, 0),
                          Token::new(String::from("a"), BASE_NONE, IDENTIFIER, 0, 0),
                          Token::new(String::from("="), BASE_NONE, OP_ASSIGN, 0, 0),
                          Token::new(String::from("5"), BASE_VALUE, NUM_INT, 0, 0),
                          Token::new(String::from(";"), BASE_NONE, SEMICOLON, 0, 0)];
    let stream = vec!["while", "(", "a", "==", "a", ")", "== true", "{", "a", "=", "5", ";", "}"];

    assert_eq!(stream, parse_while(&tok_vector));
}

#[test]
fn test_parse_while_infinite() {
    let tok_vector = vec![Token::new(String::from("while"), BASE_NONE, KEYWORD_WHILE, 0, 0),
                          Token::new(String::from("("), BASE_NONE, LEFT_BRACKET, 0, 0),
                          Token::new(String::from("1"), BASE_VALUE, NUM_INT, 0, 0),
                          Token::new(String::from(")"), BASE_NONE, RIGHT_BRACKET, 0, 0),
                          Token::new(String::from("{"), BASE_NONE, LEFT_CBRACE, 0, 0),
                          Token::new(String::from("/*Do something here*/"),
                                     BASE_COMMENT,
                                     COMMENT_MULTI,
                                     0,
                                     0),
                          Token::new(String::from("}"), BASE_NONE, RIGHT_CBRACE, 0, 0)];
    let stream = vec!["loop", "{", "/*Do something here*/\n", "}"];

    assert_eq!(stream, parse_while(&tok_vector));
}

// do-while
#[test]
fn test_parse_dowhile() {
    let tok_vector = vec![Token::new(String::from("do"), BASE_NONE, KEYWORD_DO, 0, 0),
                          Token::new(String::from("{"), BASE_NONE, LEFT_CBRACE, 0, 0),
                          Token::new(String::from("/*Do something here*/"),
                                     BASE_COMMENT,
                                     COMMENT_MULTI,
                                     0,
                                     0),
                          Token::new(String::from("}"), BASE_NONE, RIGHT_CBRACE, 0, 0),
                          Token::new(String::from("while"), BASE_NONE, KEYWORD_WHILE, 0, 0),
                          Token::new(String::from("("), BASE_NONE, LEFT_BRACKET, 0, 0),
                          Token::new(String::from("a"), BASE_NONE, IDENTIFIER, 0, 0),
                          Token::new(String::from("=="), BASE_BINOP, OP_EQU, 0, 0),
                          Token::new(String::from("a"), BASE_NONE, IDENTIFIER, 0, 0),
                          Token::new(String::from(")"), BASE_NONE, RIGHT_BRACKET, 0, 0),
                          Token::new(String::from(";"), BASE_NONE, SEMICOLON, 0, 0)];
    let stream = vec!["while",
                      "{",
                      "/*Do something here*/\n",
                      "(",
                      "a",
                      "==",
                      "a",
                      ")",
                      "== true",
                      "}",
                      "{",
                      "}",
                      ";"];

    assert_eq!(stream, parse_dowhile(&tok_vector));
}

#[test]
fn test_parse_dowhile_infinite() {
    let tok_vector = vec![Token::new(String::from("do"), BASE_NONE, KEYWORD_DO, 0, 0),
                          Token::new(String::from("{"), BASE_NONE, LEFT_CBRACE, 0, 0),
                          Token::new(String::from("/*Do something here*/"),
                                     BASE_COMMENT,
                                     COMMENT_MULTI,
                                     0,
                                     0),
                          Token::new(String::from("}"), BASE_NONE, RIGHT_CBRACE, 0, 0),
                          Token::new(String::from("while"), BASE_NONE, KEYWORD_WHILE, 0, 0),
                          Token::new(String::from("("), BASE_NONE, LEFT_BRACKET, 0, 0),
                          Token::new(String::from("1"), BASE_VALUE, NUM_INT, 0, 0),
                          Token::new(String::from(")"), BASE_NONE, RIGHT_BRACKET, 0, 0),
                          Token::new(String::from(";"), BASE_NONE, SEMICOLON, 0, 0)];
    let stream = vec!["loop", "{", "/*Do something here*/\n", "}", ";"];

    assert_eq!(stream, parse_dowhile(&tok_vector));
}

// for loop
#[test]
fn test_parse_for_braces() {
    let tok_vector = vec![Token::new(String::from("for"), BASE_NONE, KEYWORD_FOR, 0, 0),
                          Token::new(String::from("("), BASE_NONE, LEFT_BRACKET, 0, 0),
                          Token::new(String::from("i"), BASE_NONE, IDENTIFIER, 0, 0),
                          Token::new(String::from("="), BASE_NONE, OP_ASSIGN, 0, 0),
                          Token::new(String::from("0"), BASE_VALUE, NUM_INT, 0, 0),
                          Token::new(String::from(";"), BASE_NONE, SEMICOLON, 0, 0),
                          Token::new(String::from("i"), BASE_NONE, IDENTIFIER, 0, 0),
                          Token::new(String::from("<"), BASE_NONE, OP_LT, 0, 0),
                          Token::new(String::from("23"), BASE_VALUE, NUM_INT, 0, 0),
                          Token::new(String::from(";"), BASE_NONE, SEMICOLON, 0, 0),
                          Token::new(String::from("i"), BASE_NONE, IDENTIFIER, 0, 0),
                          Token::new(String::from("++"), BASE_UNOP, OP_INC, 0, 0),
                          Token::new(String::from(")"), BASE_NONE, RIGHT_BRACKET, 0, 0),
                          Token::new(String::from("{"), BASE_NONE, LEFT_CBRACE, 0, 0),
                          Token::new(String::from("func"), BASE_NONE, IDENTIFIER, 0, 0),
                          Token::new(String::from("("), BASE_NONE, LEFT_BRACKET, 0, 0),
                          Token::new(String::from(")"), BASE_NONE, RIGHT_BRACKET, 0, 0),
                          Token::new(String::from(";"), BASE_NONE, SEMICOLON, 0, 0),
                          Token::new(String::from("}"), BASE_NONE, RIGHT_CBRACE, 0, 0)];
    let stream = vec!["i", "=", "0", ";", "while", "i", "<", "23", "{", "func", "(", ")", ";",
                      "i", "+=1", ";", "}"];

    assert_eq!(stream, parse_for(&tok_vector));
}

#[test]
fn test_parse_for_init_decl() {
    let doc = STRICT;
    let tok_vector = vec![Token::new(String::from("for"), BASE_NONE, KEYWORD_FOR, 0, 0),
                          Token::new(String::from("("), BASE_NONE, LEFT_BRACKET, 0, 0),
                          Token::new(String::from("int"), BASE_DATATYPE, PRIMITIVE_INT, 0, 0),
                          Token::new(String::from("i"), BASE_NONE, IDENTIFIER, 0, 0),
                          Token::new(String::from("="), BASE_NONE, OP_ASSIGN, 0, 0),
                          Token::new(String::from("0"), BASE_VALUE, NUM_INT, 0, 0),
                          Token::new(String::from(";"), BASE_NONE, SEMICOLON, 0, 0),
                          Token::new(String::from("i"), BASE_NONE, IDENTIFIER, 0, 0),
                          Token::new(String::from("<"), BASE_NONE, OP_LT, 0, 0),
                          Token::new(String::from("23"), BASE_VALUE, NUM_INT, 0, 0),
                          Token::new(String::from(";"), BASE_NONE, SEMICOLON, 0, 0),
                          Token::new(String::from("i"), BASE_NONE, IDENTIFIER, 0, 0),
                          Token::new(String::from("++"), BASE_UNOP, OP_INC, 0, 0),
                          Token::new(String::from(")"), BASE_NONE, RIGHT_BRACKET, 0, 0),
                          Token::new(String::from("{"), BASE_NONE, LEFT_CBRACE, 0, 0),
                          Token::new(String::from("func"), BASE_NONE, IDENTIFIER, 0, 0),
                          Token::new(String::from("("), BASE_NONE, LEFT_BRACKET, 0, 0),
                          Token::new(String::from(")"), BASE_NONE, RIGHT_BRACKET, 0, 0),
                          Token::new(String::from(";"), BASE_NONE, SEMICOLON, 0, 0),
                          Token::new(String::from("}"), BASE_NONE, RIGHT_CBRACE, 0, 0)];
    let stream = vec![doc.get_doc(),
                      "static",
                      "i",
                      ":",
                      "i32",
                      "=",
                      "0",
                      ";",
                      "while",
                      "i",
                      "<",
                      "23",
                      "{",
                      "func",
                      "(",
                      ")",
                      ";",
                      "i",
                      "+=1",
                      ";",
                      "}"];

    assert_eq!(stream, parse_for(&tok_vector));
}

#[test]
fn test_parse_for_no_init() {
    let tok_vector = vec![Token::new(String::from("for"), BASE_NONE, KEYWORD_FOR, 0, 0),
                          Token::new(String::from("("), BASE_NONE, LEFT_BRACKET, 0, 0),
                          Token::new(String::from(";"), BASE_NONE, SEMICOLON, 0, 0),
                          Token::new(String::from("i"), BASE_NONE, IDENTIFIER, 0, 0),
                          Token::new(String::from("<"), BASE_NONE, OP_LT, 0, 0),
                          Token::new(String::from("23"), BASE_VALUE, NUM_INT, 0, 0),
                          Token::new(String::from(";"), BASE_NONE, SEMICOLON, 0, 0),
                          Token::new(String::from("i"), BASE_NONE, IDENTIFIER, 0, 0),
                          Token::new(String::from("++"), BASE_UNOP, OP_INC, 0, 0),
                          Token::new(String::from(")"), BASE_NONE, RIGHT_BRACKET, 0, 0),
                          Token::new(String::from("{"), BASE_NONE, LEFT_CBRACE, 0, 0),
                          Token::new(String::from("func"), BASE_NONE, IDENTIFIER, 0, 0),
                          Token::new(String::from("("), BASE_NONE, LEFT_BRACKET, 0, 0),
                          Token::new(String::from(")"), BASE_NONE, RIGHT_BRACKET, 0, 0),
                          Token::new(String::from(";"), BASE_NONE, SEMICOLON, 0, 0),
                          Token::new(String::from("}"), BASE_NONE, RIGHT_CBRACE, 0, 0)];
    let stream = vec!["while", "i", "<", "23", "{", "func", "(", ")", ";", "i", "+=1", ";", "}"];

    assert_eq!(stream, parse_for(&tok_vector));
}

#[test]
fn test_parse_for_no_cond() {
    let tok_vector = vec![Token::new(String::from("for"), BASE_NONE, KEYWORD_FOR, 0, 0),
                          Token::new(String::from("("), BASE_NONE, LEFT_BRACKET, 0, 0),
                          Token::new(String::from("i"), BASE_NONE, IDENTIFIER, 0, 0),
                          Token::new(String::from("="), BASE_NONE, OP_ASSIGN, 0, 0),
                          Token::new(String::from("0"), BASE_VALUE, NUM_INT, 0, 0),
                          Token::new(String::from(";"), BASE_NONE, SEMICOLON, 0, 0),
                          Token::new(String::from(";"), BASE_NONE, SEMICOLON, 0, 0),
                          Token::new(String::from("i"), BASE_NONE, IDENTIFIER, 0, 0),
                          Token::new(String::from("++"), BASE_UNOP, OP_INC, 0, 0),
                          Token::new(String::from(")"), BASE_NONE, RIGHT_BRACKET, 0, 0),
                          Token::new(String::from("{"), BASE_NONE, LEFT_CBRACE, 0, 0),
                          Token::new(String::from("func"), BASE_NONE, IDENTIFIER, 0, 0),
                          Token::new(String::from("("), BASE_NONE, LEFT_BRACKET, 0, 0),
                          Token::new(String::from(")"), BASE_NONE, RIGHT_BRACKET, 0, 0),
                          Token::new(String::from(";"), BASE_NONE, SEMICOLON, 0, 0),
                          Token::new(String::from("}"), BASE_NONE, RIGHT_CBRACE, 0, 0)];
    let stream = vec!["i", "=", "0", ";", "loop", "{", "func", "(", ")", ";", "i", "+=1", ";", "}"];

    assert_eq!(stream, parse_for(&tok_vector));
}

#[test]
fn test_parse_for_no_update() {
    let tok_vector = vec![Token::new(String::from("for"), BASE_NONE, KEYWORD_FOR, 0, 0),
                          Token::new(String::from("("), BASE_NONE, LEFT_BRACKET, 0, 0),
                          Token::new(String::from("i"), BASE_NONE, IDENTIFIER, 0, 0),
                          Token::new(String::from("="), BASE_NONE, OP_ASSIGN, 0, 0),
                          Token::new(String::from("0"), BASE_VALUE, NUM_INT, 0, 0),
                          Token::new(String::from(";"), BASE_NONE, SEMICOLON, 0, 0),
                          Token::new(String::from("i"), BASE_NONE, IDENTIFIER, 0, 0),
                          Token::new(String::from("<"), BASE_NONE, OP_LT, 0, 0),
                          Token::new(String::from("23"), BASE_VALUE, NUM_INT, 0, 0),
                          Token::new(String::from(";"), BASE_NONE, SEMICOLON, 0, 0),
                          Token::new(String::from(")"), BASE_NONE, RIGHT_BRACKET, 0, 0),
                          Token::new(String::from("{"), BASE_NONE, LEFT_CBRACE, 0, 0),
                          Token::new(String::from("func"), BASE_NONE, IDENTIFIER, 0, 0),
                          Token::new(String::from("("), BASE_NONE, LEFT_BRACKET, 0, 0),
                          Token::new(String::from(")"), BASE_NONE, RIGHT_BRACKET, 0, 0),
                          Token::new(String::from(";"), BASE_NONE, SEMICOLON, 0, 0),
                          Token::new(String::from("}"), BASE_NONE, RIGHT_CBRACE, 0, 0)];
    let stream = vec!["i", "=", "0", ";", "while", "i", "<", "23", "{", "func", "(", ")", ";", "}"];

    assert_eq!(stream, parse_for(&tok_vector));
}

#[test]
fn test_parse_for_infinite() {
    let tok_vector = vec![Token::new(String::from("for"), BASE_NONE, KEYWORD_FOR, 0, 0),
                          Token::new(String::from("("), BASE_NONE, LEFT_BRACKET, 0, 0),
                          Token::new(String::from(";"), BASE_NONE, SEMICOLON, 0, 0),
                          Token::new(String::from(";"), BASE_NONE, SEMICOLON, 0, 0),
                          Token::new(String::from(")"), BASE_NONE, RIGHT_BRACKET, 0, 0),
                          Token::new(String::from("{"), BASE_NONE, LEFT_CBRACE, 0, 0),
                          Token::new(String::from("func"), BASE_NONE, IDENTIFIER, 0, 0),
                          Token::new(String::from("("), BASE_NONE, LEFT_BRACKET, 0, 0),
                          Token::new(String::from(")"), BASE_NONE, RIGHT_BRACKET, 0, 0),
                          Token::new(String::from(";"), BASE_NONE, SEMICOLON, 0, 0),
                          Token::new(String::from("}"), BASE_NONE, RIGHT_CBRACE, 0, 0)];
    let stream = vec!["loop", "{", "func", "(", ")", ";", "}"];

    assert_eq!(stream, parse_for(&tok_vector));
}

#[test]
fn test_parse_for_no_braces() {
    let tok_vector = vec![Token::new(String::from("for"), BASE_NONE, KEYWORD_FOR, 0, 0),
                          Token::new(String::from("("), BASE_NONE, LEFT_BRACKET, 0, 0),
                          Token::new(String::from("i"), BASE_NONE, IDENTIFIER, 0, 0),
                          Token::new(String::from("="), BASE_NONE, OP_ASSIGN, 0, 0),
                          Token::new(String::from("0"), BASE_VALUE, NUM_INT, 0, 0),
                          Token::new(String::from(";"), BASE_NONE, SEMICOLON, 0, 0),
                          Token::new(String::from("i"), BASE_NONE, IDENTIFIER, 0, 0),
                          Token::new(String::from("<"), BASE_NONE, OP_LT, 0, 0),
                          Token::new(String::from("23"), BASE_VALUE, NUM_INT, 0, 0),
                          Token::new(String::from(";"), BASE_NONE, SEMICOLON, 0, 0),
                          Token::new(String::from("i"), BASE_NONE, IDENTIFIER, 0, 0),
                          Token::new(String::from("++"), BASE_UNOP, OP_INC, 0, 0),
                          Token::new(String::from(")"), BASE_NONE, RIGHT_BRACKET, 0, 0),
                          Token::new(String::from("func"), BASE_NONE, IDENTIFIER, 0, 0),
                          Token::new(String::from("("), BASE_NONE, LEFT_BRACKET, 0, 0),
                          Token::new(String::from(")"), BASE_NONE, RIGHT_BRACKET, 0, 0),
                          Token::new(String::from(";"), BASE_NONE, SEMICOLON, 0, 0)];
    let stream = vec!["i", "=", "0", ";", "while", "i", "<", "23", "{", "func", "(", ")", ";",
                      "i", "+=1", ";", "}"];

    assert_eq!(stream, parse_for(&tok_vector));
}

//switch
#[test]
fn test_parse_switch_no_braces() {
    let tok_vector = vec![Token::new(String::from("switch"), BASE_NONE, KEYWORD_SWITCH, 0, 0),
                          Token::new(String::from("("), BASE_NONE, LEFT_BRACKET, 0, 0),
                          Token::new(String::from("i"), BASE_NONE, IDENTIFIER, 0, 0),
                          Token::new(String::from(")"), BASE_NONE, RIGHT_BRACKET, 0, 0),
                          Token::new(String::from("{"), BASE_NONE, LEFT_CBRACE, 0, 0),
                          Token::new(String::from("case"), BASE_NONE, KEYWORD_CASE, 0, 0),
                          Token::new(String::from("23"), BASE_VALUE, NUM_INT, 0, 0),
                          Token::new(String::from(":"), BASE_NONE, COLON, 0, 0),
                          Token::new(String::from("i"), BASE_NONE, IDENTIFIER, 0, 0),
                          Token::new(String::from("++"), BASE_UNOP, OP_INC, 0, 0),
                          Token::new(String::from(";"), BASE_NONE, SEMICOLON, 0, 0),
                          Token::new(String::from("func"), BASE_NONE, IDENTIFIER, 0, 0),
                          Token::new(String::from("("), BASE_NONE, LEFT_BRACKET, 0, 0),
                          Token::new(String::from(")"), BASE_NONE, RIGHT_BRACKET, 0, 0),
                          Token::new(String::from(";"), BASE_NONE, SEMICOLON, 0, 0),
                          Token::new(String::from("default"), BASE_NONE, KEYWORD_DEFAULT, 0, 0),
                          Token::new(String::from(":"), BASE_NONE, COLON, 0, 0),
                          Token::new(String::from("i"), BASE_NONE, IDENTIFIER, 0, 0),
                          Token::new(String::from("--"), BASE_UNOP, OP_DEC, 0, 0),
                          Token::new(String::from(";"), BASE_NONE, SEMICOLON, 0, 0),
                          Token::new(String::from("}"), BASE_NONE, RIGHT_CBRACE, 0, 0)];
    let stream = vec!["match", "i", "{", "23", "=>", "{", "i", "+=1", ";", "func", "(", ")", ";",
                      "}", "_", "=>", "{", "i", "-=1", ";", "}", "}"];
    assert_eq!(stream, parse_switch(&tok_vector));
}

#[test]
fn test_parse_switch_mix_braces() {
    let tok_vector = vec![Token::new(String::from("switch"), BASE_NONE, KEYWORD_SWITCH, 0, 0),
                          Token::new(String::from("("), BASE_NONE, LEFT_BRACKET, 0, 0),
                          Token::new(String::from("i"), BASE_NONE, IDENTIFIER, 0, 0),
                          Token::new(String::from(")"), BASE_NONE, RIGHT_BRACKET, 0, 0),
                          Token::new(String::from("{"), BASE_NONE, LEFT_CBRACE, 0, 0),
                          Token::new(String::from("case"), BASE_NONE, KEYWORD_CASE, 0, 0),
                          Token::new(String::from("23"), BASE_VALUE, NUM_INT, 0, 0),
                          Token::new(String::from(":"), BASE_NONE, COLON, 0, 0),
                          Token::new(String::from("i"), BASE_NONE, IDENTIFIER, 0, 0),
                          Token::new(String::from("++"), BASE_UNOP, OP_INC, 0, 0),
                          Token::new(String::from(";"), BASE_NONE, SEMICOLON, 0, 0),
                          Token::new(String::from("func"), BASE_NONE, IDENTIFIER, 0, 0),
                          Token::new(String::from("("), BASE_NONE, LEFT_BRACKET, 0, 0),
                          Token::new(String::from(")"), BASE_NONE, RIGHT_BRACKET, 0, 0),
                          Token::new(String::from(";"), BASE_NONE, SEMICOLON, 0, 0),
                          Token::new(String::from("default"), BASE_NONE, KEYWORD_DEFAULT, 0, 0),
                          Token::new(String::from(":"), BASE_NONE, COLON, 0, 0),
                          Token::new(String::from("{"), BASE_NONE, LEFT_CBRACE, 0, 0),
                          Token::new(String::from("i"), BASE_NONE, IDENTIFIER, 0, 0),
                          Token::new(String::from("--"), BASE_UNOP, OP_DEC, 0, 0),
                          Token::new(String::from(";"), BASE_NONE, SEMICOLON, 0, 0),
                          Token::new(String::from("}"), BASE_NONE, RIGHT_CBRACE, 0, 0),
                          Token::new(String::from("}"), BASE_NONE, RIGHT_CBRACE, 0, 0)];
    let stream = vec!["match", "i", "{", "23", "=>", "{", "i", "+=1", ";", "func", "(", ")", ";",
                      "}", "_", "=>", "{", "i", "-=1", ";", "}", "}"];
    assert_eq!(stream, parse_switch(&tok_vector));
}

#[test]
fn test_parse_switch_empty() {
    let tok_vector = vec![Token::new(String::from("switch"), BASE_NONE, KEYWORD_SWITCH, 0, 0),
                          Token::new(String::from("("), BASE_NONE, LEFT_BRACKET, 0, 0),
                          Token::new(String::from("i"), BASE_NONE, IDENTIFIER, 0, 0),
                          Token::new(String::from(")"), BASE_NONE, RIGHT_BRACKET, 0, 0),
                          Token::new(String::from("{"), BASE_NONE, LEFT_CBRACE, 0, 0),
                          Token::new(String::from("}"), BASE_NONE, RIGHT_CBRACE, 0, 0)];
    let stream = vec!["match", "i", "{", "_", "=>", "{", "}", "}"];
    assert_eq!(stream, parse_switch(&tok_vector));
}

#[test]
fn test_parse_switch_expr() {
    let mut tok_vector = vec![Token::new(String::from("switch"), BASE_NONE, KEYWORD_SWITCH, 0, 0),
                              Token::new(String::from("("), BASE_NONE, LEFT_BRACKET, 0, 0),
                              Token::new(String::from("fun"), BASE_NONE, IDENTIFIER, 0, 0),
                              Token::new(String::from("("), BASE_NONE, LEFT_BRACKET, 0, 0),
                              Token::new(String::from("i"), BASE_NONE, IDENTIFIER, 0, 0),
                              Token::new(String::from(","), BASE_NONE, COMMA, 0, 0),
                              Token::new(String::from("j"), BASE_NONE, IDENTIFIER, 0, 0),
                              Token::new(String::from(")"), BASE_NONE, RIGHT_BRACKET, 0, 0),
                              Token::new(String::from(")"), BASE_NONE, RIGHT_BRACKET, 0, 0),
                              Token::new(String::from("{"), BASE_NONE, LEFT_CBRACE, 0, 0),
                              Token::new(String::from("case"), BASE_NONE, KEYWORD_CASE, 0, 0),
                              Token::new(String::from("23"), BASE_VALUE, NUM_INT, 0, 0),
                              Token::new(String::from(":"), BASE_NONE, COLON, 0, 0),
                              Token::new(String::from("{"), BASE_NONE, LEFT_CBRACE, 0, 0),
                              Token::new(String::from("i"), BASE_NONE, IDENTIFIER, 0, 0),
                              Token::new(String::from("++"), BASE_UNOP, OP_INC, 0, 0),
                              Token::new(String::from(";"), BASE_NONE, SEMICOLON, 0, 0),
                              Token::new(String::from("func"), BASE_NONE, IDENTIFIER, 0, 0),
                              Token::new(String::from("("), BASE_NONE, LEFT_BRACKET, 0, 0),
                              Token::new(String::from(")"), BASE_NONE, RIGHT_BRACKET, 0, 0),
                              Token::new(String::from(";"), BASE_NONE, SEMICOLON, 0, 0),
                              Token::new(String::from("}"), BASE_NONE, RIGHT_CBRACE, 0, 0),
                              Token::new(String::from("}"), BASE_NONE, RIGHT_CBRACE, 0, 0)];
    let mut stream = vec!["match", "fun", "(", "i", ",", "j", ")", "{", "23", "=>", "{", "i",
                          "+=1", ";", "func", "(", ")", ";", "}", "_", "=>", "{", "}", "}"];
    assert_eq!(stream, parse_switch(&tok_vector));

    tok_vector = vec![Token::new(String::from("switch"), BASE_NONE, KEYWORD_SWITCH, 0, 0),
                      Token::new(String::from("("), BASE_NONE, LEFT_BRACKET, 0, 0),
                      Token::new(String::from("arr"), BASE_NONE, IDENTIFIER, 0, 0),
                      Token::new(String::from("["), BASE_NONE, LEFT_BRACKET, 0, 0),
                      Token::new(String::from("i"), BASE_NONE, IDENTIFIER, 0, 0),
                      Token::new(String::from("]"), BASE_NONE, RIGHT_BRACKET, 0, 0),
                      Token::new(String::from(")"), BASE_NONE, RIGHT_BRACKET, 0, 0),
                      Token::new(String::from("{"), BASE_NONE, LEFT_CBRACE, 0, 0),
                      Token::new(String::from("case"), BASE_NONE, KEYWORD_CASE, 0, 0),
                      Token::new(String::from("23"), BASE_VALUE, NUM_INT, 0, 0),
                      Token::new(String::from(":"), BASE_NONE, COLON, 0, 0),
                      Token::new(String::from("{"), BASE_NONE, LEFT_CBRACE, 0, 0),
                      Token::new(String::from("i"), BASE_NONE, IDENTIFIER, 0, 0),
                      Token::new(String::from("++"), BASE_UNOP, OP_INC, 0, 0),
                      Token::new(String::from(";"), BASE_NONE, SEMICOLON, 0, 0),
                      Token::new(String::from("func"), BASE_NONE, IDENTIFIER, 0, 0),
                      Token::new(String::from("("), BASE_NONE, LEFT_BRACKET, 0, 0),
                      Token::new(String::from(")"), BASE_NONE, RIGHT_BRACKET, 0, 0),
                      Token::new(String::from(";"), BASE_NONE, SEMICOLON, 0, 0),
                      Token::new(String::from("}"), BASE_NONE, RIGHT_CBRACE, 0, 0),
                      Token::new(String::from("}"), BASE_NONE, RIGHT_CBRACE, 0, 0)];
    stream = vec!["match", "arr", "[", "i", "]", "{", "23", "=>", "{", "i", "+=1", ";", "func",
                  "(", ")", ";", "}", "_", "=>", "{", "}", "}"];
    assert_eq!(stream, parse_switch(&tok_vector));
}

#[test]
fn test_parse_switch_no_default() {
    let tok_vector = vec![Token::new(String::from("switch"), BASE_NONE, KEYWORD_SWITCH, 0, 0),
                          Token::new(String::from("("), BASE_NONE, LEFT_BRACKET, 0, 0),
                          Token::new(String::from("i"), BASE_NONE, IDENTIFIER, 0, 0),
                          Token::new(String::from(")"), BASE_NONE, RIGHT_BRACKET, 0, 0),
                          Token::new(String::from("{"), BASE_NONE, LEFT_CBRACE, 0, 0),
                          Token::new(String::from("case"), BASE_NONE, KEYWORD_CASE, 0, 0),
                          Token::new(String::from("23"), BASE_VALUE, NUM_INT, 0, 0),
                          Token::new(String::from(":"), BASE_NONE, COLON, 0, 0),
                          Token::new(String::from("{"), BASE_NONE, LEFT_CBRACE, 0, 0),
                          Token::new(String::from("i"), BASE_NONE, IDENTIFIER, 0, 0),
                          Token::new(String::from("++"), BASE_UNOP, OP_INC, 0, 0),
                          Token::new(String::from(";"), BASE_NONE, SEMICOLON, 0, 0),
                          Token::new(String::from("func"), BASE_NONE, IDENTIFIER, 0, 0),
                          Token::new(String::from("("), BASE_NONE, LEFT_BRACKET, 0, 0),
                          Token::new(String::from(")"), BASE_NONE, RIGHT_BRACKET, 0, 0),
                          Token::new(String::from(";"), BASE_NONE, SEMICOLON, 0, 0),
                          Token::new(String::from("}"), BASE_NONE, RIGHT_CBRACE, 0, 0),
                          Token::new(String::from("}"), BASE_NONE, RIGHT_CBRACE, 0, 0)];
    let stream = vec!["match", "i", "{", "23", "=>", "{", "i", "+=1", ";", "func", "(", ")", ";",
                      "}", "_", "=>", "{", "}", "}"];
    assert_eq!(stream, parse_switch(&tok_vector));
}

#[test]
fn test_parse_type() {
    assert_eq!(parse_type(PRIMITIVE_INT as i32).unwrap_or(String::from("")),
               "i32");
    assert_eq!(parse_type(PRIMITIVE_CHAR as i32).unwrap_or(String::from("")),
               "char");
    assert_eq!(parse_type(PRIMITIVE_FLOAT as i32).unwrap_or(String::from("")),
               "f32");
    assert_eq!(parse_type(PRIMITIVE_DOUBLE as i32).unwrap_or(String::from("")),
               "f64");
    assert_eq!(parse_type(PRIMITIVE_SHORT as i32).unwrap_or(String::from("")),
               "i16");
    assert_eq!(parse_type(PRIMITIVE_LONG as i32).unwrap_or(String::from("")),
               "i64");
    assert_eq!(parse_type(PRIMITIVE_VOID as i32).unwrap_or(String::from("")),
               "void");
    assert_eq!(parse_type(PRIMITIVE_BOOL as i32).unwrap_or(String::from("")),
               "bool");
    assert_eq!(parse_type(OTHER as i32).unwrap_or(String::from("")), "");
}
