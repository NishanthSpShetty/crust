use crate::library::lexeme::definition::TokenType::*;
use crate::library::lexeme::token::Token;

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

pub fn get_operator_as_fucn_name(token: &Token) -> &str {
    match token.get_token_type() {
        Plus => "_plus",
        Minus => "_minus",
        Multiplication => "_mult",
        Divide => "_div",
        PlusEqual => "_plus_eq",
        MinusEqual => "_minus_eq",
        MultiplyEqual => "_mult_eq",
        DivideEqual => "_div_eq",
        Increment => "_inc",
        Decrement => "_dec",
        Equal => "_eq",
        Assignment => "_assign",
        RightBracket => "_call",
        _ => "_misc_op",
    }
}
