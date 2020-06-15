use library::lexeme::definition::TokenType;
use library::lexeme::definition::TokenType::*;

#[derive(Copy, Clone, Debug)]
pub enum Modifier {
    Unsigned,
    Signed,
    Const,
    Static,
    Default, //none applied
}


/**
 * fn parse_type:
 * takes the integer value of type Type
 * returns either the equivalent Rust type as a string or
 * None, if does not correspond to any c/c++ type
 */
pub fn parse_type(c_type: TokenType, modifier: Modifier) -> Option<String> {
    match (modifier, c_type) {
        //unsigned types
        (Modifier::Unsigned, Character) => Some("u8".to_string()),
        (Modifier::Unsigned, Short) => Some("u16".to_string()),
        (Modifier::Unsigned, Integer) => Some("u32".to_string()),
        (Modifier::Unsigned, Long) => Some("u64".to_string()),

        //signed types
        (_, Short) => Some("i16".to_string()),
        (_, Integer) => Some("i32".to_string()),
        (_, Long) => Some("i64".to_string()),

        //type without modifiers
        (_, Float) => Some("f32".to_string()),
        (_, Double) => Some("f64".to_string()),
        (_, Character) => Some("char".to_string()),
        (_, Boolean) => Some("bool".to_string()),
        (_, Void) => Some("void".to_string()),
        (_, Auto) => Some("_".to_string()),
        (_, StringValue) => Some("String".to_string()),
        (_, _) => None,
    }
}


pub fn get_default_value_for(c_type: TokenType) -> String {
    let value = match c_type {
        Integer => "0i32",
        Short => "0i16",
        Long => "0i64",
        Float => "0.0f32",
        Double => "0.0f64",
        Character => "'_'",
        Boolean => "false",
        _ => "_",
    };
    String::from(value)
}