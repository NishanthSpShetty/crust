use library::lexeme::definition::TokenType;
use library::lexeme::definition::TokenType::*;
use library::lexeme::token::Token;


/**
 * fn parse_type:
 * takes the integer value of type Type
 * returns either the equivalent Rust type as a string or
 * None, if does not correspond to any c/c++ type
 */
fn parse_type(c_type: TokenType) -> Option<String> {
    match c_type {
        Integer => Some("i32".to_string()),
        Short => Some("i16".to_string()),
        Long => Some("i64".to_string()),
        Float => Some("f32".to_string()),
        Double => Some("f64".to_string()),
        Character => Some("char".to_string()),
        Boolean => Some("bool".to_string()),
        Void => Some("void".to_string()),
        unsigned => Some("u32".to_string()),
        Signed => Some("u16".to_string()),
        unsigned => Some("u64".to_string()),
        Auto => Some("_".to_string()),
        // 13 => Some("u8".to_string()),
        StringValue => Some("String".to_string()),
        _ => None,
    }
}


/**
 * skip_stmt:
 * forwards the lookahead by one statement
 * returns the lookahead at the lexeme after the semi-colon
 */
pub fn skip_stmt(lexeme: &Vec<Token>, mut lookahead: usize) -> usize {
    while lexeme[lookahead].get_token_type() != Semicolon {
        lookahead += 1;
    }
    lookahead + 1
}


/**
 * skip_block:
 * forwards the lookahead by one block
 * returns the lookahead at the lexeme after the closing brace
 */
pub fn skip_block(lexeme: &Vec<Token>, mut lookahead: usize) -> usize {
    let mut paren = 1;

    // while all braces are not closed
    // skip nested blocks if any
    while paren != 0 && lookahead < lexeme.len() {
        if lexeme[lookahead].get_token_type() == LeftCurlyBrace {
            paren += 1;
        }
        if lexeme[lookahead].get_token_type() == RightCurlyBrace {
            paren -= 1;
        }
        lookahead += 1;
    }
    lookahead
}
