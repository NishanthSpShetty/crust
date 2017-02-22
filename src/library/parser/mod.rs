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

static mut IN_BLOCK_STMNT: bool = false;
static mut IN_EXPR: bool = false;

/**
 * parse_program:
 * parse c program from Token Vector
 * return String Vector of Rust equivalent code
 * supports:
 * - method
 * - variable declaration
 * - variable assignment
 * - if
 * - comments
 */
pub fn parse_program(lexeme: &Vec<Token>) -> Vec<String> {

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
                        unsafe {
                            IN_BLOCK_STMNT = true;
                        }

                        while lexeme[lookahead].get_token_type() != LEFT_CBRACE {
                            lookahead += 1;
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
                        unsafe {
                            IN_BLOCK_STMNT = false;
                        }

                        temp_lexeme.clear();
                    }

                    // variable declaration or assignment
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

                    _ => {}
                };
            }

            // matches if statement
            (_, KEYWORD_IF) => {
                let mut temp_lexeme: Vec<Token> = Vec::new();

                // move lookahead past conditon
                while lexeme[lookahead].get_token_type() != RIGHT_BRACKET {
                    lookahead += 1;
                }
                lookahead += 1;

                // move lookahead past block
                if lexeme[lookahead].get_token_type() == LEFT_CBRACE {
                    unsafe {
                        IN_BLOCK_STMNT = true;
                    }
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
            }

            // matches single and multi-line comment
            (BASE_COMMENT, _) => {
                stream.push(lexeme[head].get_token_value() + "\n");
                head += 1;
                lookahead = head;
            }

            // assignment statements
            (_, IDENTIFIER) => {
                let mut temp_lexeme: Vec<Token> = Vec::new();
                //identifier = expr
                //identifier()
                //identifier+expr
                //identifier OP_INC|OP_DEC; =>postfix


                match lexeme[head + 1].get_type() {
                    (_, OP_ASSIGN) => {
                        // move lookahead past statement
                        if lexeme[head + 3].get_token_type() == COMMA {
                            lookahead = head + 3;
                            while head < lookahead + 1 {
                                let l: Token = lexeme[head].clone();
                                temp_lexeme.push(l);
                                head += 1;
                            }
                            // println!("{:?}", stream);
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
                        // move lookahead past statement
                        lookahead = skip_stmt(&lexeme, lookahead);
                        // collect statement
                        while head < lookahead {
                            let l: Token = lexeme[head].clone();
                            temp_lexeme.push(l);
                            head += 1;
                        }

                        // parse assignment
                        stream.append(&mut parse_expr(&temp_lexeme));

                    }
                    (_, _) => {
                        if lexeme[head].get_token_type() != RIGHT_CBRACE {
                            stream.push(lexeme[head].get_token_value());
                        }
                        head += 1;
                        lookahead = head;
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
            // if all fails
            (_, _) => {
                if lexeme[head].get_token_type() != RIGHT_CBRACE {

                    stream.push(lexeme[head].get_token_value());
                }
                head += 1;
                lookahead = head;
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
        stream.push("->".to_string());

        // parse return type
        if let Some(rust_type) = parse_type(lexeme[0].get_token_type() as i32) {
            stream.push(rust_type);
        }

        stream.push("{".to_string());
    }
    // declare argc and argv inside main, if required
    else {
        stream.push(")".to_string());
        stream.push("{".to_string());
        if lexeme[head].get_token_type() != RIGHT_BRACKET {
            stream.push("let mut argv = env::args();".to_string());
            stream.push("let mut argc = argv.len();".to_string());
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
        unsafe {
            if IN_BLOCK_STMNT == false {
                stream.push("static".to_string());
            } else {
                stream.push("let".to_string());
            }
        }
        stream.push(i.id_name.clone());
        stream.push(":".to_string());

        // get the rust type
        if let Some(rust_type) = parse_type(i.typ) {
            stream.push(rust_type);
        } else {
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
 * parse_if:
 * parse c/c++ if statements into rust
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
        head += 1;
    }
    head += 1;

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

    let n = 2;
    let m = 3;

    let mut tstream: Vec<String> = Vec::new();

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
        //  println!("found bin BASE_BINOP");
        while lexeme[thead].get_token_type() != SEMICOLON {
            lexeme1.push(lexeme[thead].clone());
            thead += 1;
        }
        lexeme1.push(lexeme[thead].clone());
        stream.push(lexeme[0].get_token_value());
        stream.push(lexeme[1].get_token_value());
        stream.append(&mut parse_expr(&lexeme1));
    } else {
        if lexeme[m].get_token_type() != SEMICOLON && lexeme[m].get_token_type() != COMMA {
            while lexeme[thead].get_token_type() != SEMICOLON &&
                  lexeme[thead].get_token_type() != COMMA {
                lexeme1.push(lexeme[thead].clone());
                thead += 1;
            }
            lexeme1.push(lexeme[thead].clone());
            print_lexemes(&lexeme1, 0, lexeme1.len());
            stream.append(&mut parse_program(&lexeme1));
            println!("{:?}", stream);
        }
        stream.push(lexeme[0].get_token_value());
        stream.push(lexeme[1].get_token_value());
        if lexeme[n].get_base_type() == BASE_UNOP {
            stream.push(lexeme[m].get_token_value());
        } else {
            stream.push(lexeme[n].get_token_value());
        }
        stream.push(";".to_string());

    }
    if tstream.len() > 0 {
        stream.append(&mut tstream);
    }
    stream
}


/* parse_assignment:
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

            if lexeme[thead].get_base_type() != BASE_UNOP {
                stream.push(lexeme[thead].get_token_value());
            }
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

#[test]
fn test_parse_if_braces() {
    let tok_vector = vec![Token::new(String::from("if"), BASE_NONE, KEYWORD_IF, 0, 0),
                          Token::new(String::from("("), BASE_NONE, LEFT_BRACKET, 0, 1),
                          Token::new(String::from("a"), BASE_NONE, IDENTIFIER, 0, 2),
                          Token::new(String::from("=="), BASE_BINOP, OP_EQU, 0, 3),
                          Token::new(String::from("a"), BASE_NONE, IDENTIFIER, 0, 4),
                          Token::new(String::from(")"), BASE_NONE, RIGHT_BRACKET, 0, 5),
                          Token::new(String::from("{"), BASE_NONE, LEFT_CBRACE, 0, 6),
                          Token::new(String::from("/*Do something here*/"),
                                     BASE_COMMENT,
                                     COMMENT_MULTI,
                                     1,
                                     1),
                          Token::new(String::from("}"), BASE_NONE, RIGHT_CBRACE, 2, 7)];
    let stream = vec!["if", "a", "==", "a", "{", "/*Do something here*/\n", "}"];

    assert_eq!(stream, parse_if(&tok_vector));
}

#[test]
fn test_parse_if_braces_nesting() {
    let tok_vector = vec![Token::new(String::from("if"), BASE_NONE, KEYWORD_IF, 0, 0),
                          Token::new(String::from("("), BASE_NONE, LEFT_BRACKET, 0, 1),
                          Token::new(String::from("a"), BASE_NONE, IDENTIFIER, 0, 2),
                          Token::new(String::from(">"), BASE_BINOP, OP_GT, 0, 3),
                          Token::new(String::from("2"), BASE_VALUE, NUM_INT, 0, 4),
                          Token::new(String::from(")"), BASE_NONE, RIGHT_BRACKET, 0, 5),
                          Token::new(String::from("{"), BASE_NONE, LEFT_CBRACE, 0, 6),
                          Token::new(String::from("/*Do something here*/"),
                                     BASE_COMMENT,
                                     COMMENT_MULTI,
                                     1,
                                     7),
                          Token::new(String::from("if"), BASE_NONE, KEYWORD_IF, 2, 8),
                          Token::new(String::from("("), BASE_NONE, LEFT_BRACKET, 2, 9),
                          Token::new(String::from("a"), BASE_NONE, IDENTIFIER, 2, 10),
                          Token::new(String::from("<"), BASE_BINOP, OP_LT, 2, 11),
                          Token::new(String::from("4"), BASE_VALUE, NUM_INT, 2, 12),
                          Token::new(String::from(")"), BASE_NONE, RIGHT_BRACKET, 2, 13),
                          Token::new(String::from("{"), BASE_NONE, LEFT_CBRACE, 2, 14),
                          Token::new(String::from("b"), BASE_NONE, IDENTIFIER, 3, 15),
                          Token::new(String::from("="), BASE_NONE, OP_ASSIGN, 3, 16),
                          Token::new(String::from("53"), BASE_VALUE, NUM_INT, 3, 17),
                          Token::new(String::from(";"), BASE_VALUE, SEMICOLON, 3, 18),
                          Token::new(String::from("}"), BASE_NONE, RIGHT_CBRACE, 4, 19),
                          Token::new(String::from("b"), BASE_NONE, IDENTIFIER, 5, 20),
                          Token::new(String::from("="), BASE_NONE, OP_ASSIGN, 5, 21),
                          Token::new(String::from("72"), BASE_VALUE, NUM_INT, 5, 22),
                          Token::new(String::from(";"), BASE_NONE, SEMICOLON, 5, 23),
                          Token::new(String::from("}"), BASE_NONE, RIGHT_CBRACE, 6, 24)];
    let stream = vec!["if",
                      "a",
                      ">",
                      "2",
                      "{",
                      "/*Do something here*/\n",
                      "if",
                      "a",
                      "<",
                      "4",
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
fn test_parse_if_no_braces_nesting() {
    let tok_vector = vec![Token::new(String::from("if"), BASE_NONE, KEYWORD_IF, 0, 0),
                          Token::new(String::from("("), BASE_NONE, LEFT_BRACKET, 0, 1),
                          Token::new(String::from("a"), BASE_NONE, IDENTIFIER, 0, 2),
                          Token::new(String::from(">"), BASE_BINOP, OP_GT, 0, 3),
                          Token::new(String::from("2"), BASE_VALUE, NUM_INT, 0, 4),
                          Token::new(String::from(")"), BASE_NONE, RIGHT_BRACKET, 0, 5),
                          Token::new(String::from("if"), BASE_NONE, KEYWORD_IF, 1, 7),
                          Token::new(String::from("("), BASE_NONE, LEFT_BRACKET, 1, 8),
                          Token::new(String::from("a"), BASE_NONE, IDENTIFIER, 1, 9),
                          Token::new(String::from("<"), BASE_BINOP, OP_LT, 1, 10),
                          Token::new(String::from("4"), BASE_VALUE, NUM_INT, 1, 11),
                          Token::new(String::from(")"), BASE_NONE, RIGHT_BRACKET, 1, 12),
                          Token::new(String::from("b"), BASE_NONE, IDENTIFIER, 2, 15),
                          Token::new(String::from("="), BASE_NONE, OP_ASSIGN, 2, 16),
                          Token::new(String::from("53"), BASE_VALUE, NUM_INT, 2, 17),
                          Token::new(String::from(","), BASE_NONE, COMMA, 2, 18),
                          Token::new(String::from("b"), BASE_NONE, IDENTIFIER, 3, 20),
                          Token::new(String::from("="), BASE_NONE, OP_ASSIGN, 3, 21),
                          Token::new(String::from("72"), BASE_VALUE, NUM_INT, 3, 22),
                          Token::new(String::from(";"), BASE_NONE, SEMICOLON, 3, 23)];
    let stream = vec!["if", "a", ">", "2", "{", "if", "a", "<", "4", "{", "b", "=", "53", ";",
                      "b", "=", "72", ";", "}", "}"];

    assert_eq!(stream, parse_if(&tok_vector));
}

#[test]
fn test_parse_if_no_braces() {
    let tok_vector = vec![Token::new(String::from("if"), BASE_NONE, KEYWORD_IF, 0, 0),
                          Token::new(String::from("("), BASE_NONE, LEFT_BRACKET, 0, 1),
                          Token::new(String::from("a"), BASE_NONE, IDENTIFIER, 0, 2),
                          Token::new(String::from("=="), BASE_BINOP, OP_EQU, 0, 3),
                          Token::new(String::from("a"), BASE_NONE, IDENTIFIER, 0, 4),
                          Token::new(String::from(")"), BASE_NONE, RIGHT_BRACKET, 0, 5),
                          Token::new(String::from("a"), BASE_NONE, IDENTIFIER, 1, 6),
                          Token::new(String::from("="), BASE_NONE, OP_ASSIGN, 1, 7),
                          Token::new(String::from("5"), BASE_VALUE, NUM_INT, 1, 8),
                          Token::new(String::from(";"), BASE_NONE, SEMICOLON, 1, 8)];
    let stream = vec!["if", "a", "==", "a", "{", "a", "=", "5", ";", "}"];

    assert_eq!(stream, parse_if(&tok_vector));
}
