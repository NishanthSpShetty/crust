use crate::library::lexeme::definition::TokenType;

#[derive(Copy, Clone, Debug)]
pub enum Modifier {
    Unsigned,
    Signed,
    Const,
    Static,
    Default, //none applied
}

/// Takes the integer value of type Type
/// returns either the equivalent Rust type as a string or
/// None, if does not correspond to any c/c++ type
pub fn parse_type(c_type: TokenType, modifier: Modifier) -> Option<String> {
    match (modifier, c_type) {
        //unsigned types
        (Modifier::Unsigned, TokenType::Character) => Some("u8".to_string()),
        (Modifier::Unsigned, TokenType::Short) => Some("u16".to_string()),
        (Modifier::Unsigned, TokenType::Integer) => Some("u32".to_string()),
        (Modifier::Unsigned, TokenType::Long) => Some("u64".to_string()),

        //signed types
        (_, TokenType::Short) => Some("i16".to_string()),
        (_, TokenType::Integer) => Some("i32".to_string()),
        (_, TokenType::Long) => Some("i64".to_string()),

        //type without modifiers
        (_, TokenType::Float) => Some("f32".to_string()),
        (_, TokenType::Double) => Some("f64".to_string()),
        (_, TokenType::Character) => Some("char".to_string()),
        (_, TokenType::Boolean) => Some("bool".to_string()),
        (_, TokenType::Void) => Some("void".to_string()),
        (_, TokenType::Auto) => Some("_".to_string()),
        (_, TokenType::StringValue) => Some("String".to_string()),
        (_, _) => None,
    }
}

pub fn get_default_value_for(c_type: TokenType) -> String {
    let value = match c_type {
        TokenType::Integer => "0i32",
        TokenType::Short => "0i16",
        TokenType::Long => "0i64",
        TokenType::Float => "0.0f32",
        TokenType::Double => "0.0f64",
        TokenType::Character => "'_'",
        TokenType::Boolean => "false",
        _ => "_",
    };
    String::from(value)
}
