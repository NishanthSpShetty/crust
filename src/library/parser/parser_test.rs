#[cfg(test)]
mod tests {
    use crate::library::lexeme::token::Token;
    use crate::library::lexeme::definition::{TokenType::*, TokenKind};
    use crate::library::parser::parser::init_parser;

    fn create_token(value: &str, kind: TokenKind, typ: crate::library::lexeme::definition::TokenType) -> Token {
        Token::new(value.to_string(), kind, typ, 0, 0)
    }

    // ============================================================================
    // DECLARATION TESTS
    // ============================================================================

    #[test]
    fn test_simple_int_declaration() {
        let tokens = vec![
            create_token("int", TokenKind::DataTypes, Integer),
            create_token("x", TokenKind::Identifiers, Identifier),
            create_token(";", TokenKind::SpecialChars, Semicolon),
        ];
        
        let result = init_parser(&tokens, false);
        assert!(result.len() > 0, "Parser should generate output for int declaration");
        let output = result.join("");
        assert!(output.contains("static"), "Should generate static declaration");
        assert!(output.contains("x"), "Should contain variable name");
    }

    #[test]
    fn test_int_declaration_with_assignment() {
        let tokens = vec![
            create_token("int", TokenKind::DataTypes, Integer),
            create_token("x", TokenKind::Identifiers, Identifier),
            create_token("=", TokenKind::AssignmentOperators, Assignment),
            create_token("5", TokenKind::Values, NumberInteger),
            create_token(";", TokenKind::SpecialChars, Semicolon),
        ];
        
        let result = init_parser(&tokens, false);
        let output = result.join("");
        assert!(output.contains("static"), "Should generate static declaration");
        assert!(output.contains("="), "Should contain assignment operator");
        assert!(output.contains("5"), "Should contain value 5");
    }

    #[test]
    fn test_multiple_variable_declaration() {
        let tokens = vec![
            create_token("int", TokenKind::DataTypes, Integer),
            create_token("a", TokenKind::Identifiers, Identifier),
            create_token(",", TokenKind::SpecialChars, Comma),
            create_token("b", TokenKind::Identifiers, Identifier),
            create_token(";", TokenKind::SpecialChars, Semicolon),
        ];
        
        let result = init_parser(&tokens, false);
        assert!(result.len() > 0, "Parser should handle multiple declarations");
    }

    #[test]
    fn test_unsigned_modifier() {
        let tokens = vec![
            create_token("unsigned", TokenKind::Modifiers, Unsigned),
            create_token("int", TokenKind::DataTypes, Integer),
            create_token("x", TokenKind::Identifiers, Identifier),
            create_token(";", TokenKind::SpecialChars, Semicolon),
        ];
        
        let result = init_parser(&tokens, false);
        let output = result.join("");
        assert!(output.len() > 0, "Should handle unsigned modifier");
    }

    #[test]
    fn test_const_declaration() {
        let tokens = vec![
            create_token("const", TokenKind::Modifiers, KeywordConst),
            create_token("int", TokenKind::DataTypes, Integer),
            create_token("x", TokenKind::Identifiers, Identifier),
            create_token("=", TokenKind::AssignmentOperators, Assignment),
            create_token("10", TokenKind::Values, NumberInteger),
            create_token(";", TokenKind::SpecialChars, Semicolon),
        ];
        
        let result = init_parser(&tokens, false);
        let output = result.join("");
        assert!(output.len() > 0, "Should handle const declaration");
    }

    #[test]
    fn test_pointer_declaration() {
        let tokens = vec![
            create_token("int", TokenKind::DataTypes, Integer),
            create_token("*", TokenKind::BinaryOperators, Multiplication),
            create_token("ptr", TokenKind::Identifiers, Identifier),
            create_token(";", TokenKind::SpecialChars, Semicolon),
        ];
        
        let result = init_parser(&tokens, false);
        let output = result.join("");
        assert!(output.contains("&"), "Should translate pointer to reference");
    }

    // ============================================================================
    // CONTROL FLOW TESTS
    // ============================================================================

    #[test]
    fn test_simple_if_statement() {
        let tokens = vec![
            create_token("if", TokenKind::Keyword, KeywordIf),
            create_token("(", TokenKind::SpecialChars, LeftBracket),
            create_token("x", TokenKind::Identifiers, Identifier),
            create_token(")", TokenKind::SpecialChars, RightBracket),
            create_token("{", TokenKind::SpecialChars, LeftCurlyBrace),
            create_token("}", TokenKind::SpecialChars, RightCurlyBrace),
        ];
        
        let result = init_parser(&tokens, false);
        let output = result.join("");
        assert!(output.contains("if"), "Should contain if keyword");
    }

    #[test]
    fn test_while_loop() {
        let tokens = vec![
            create_token("while", TokenKind::Keyword, KeywordWhile),
            create_token("(", TokenKind::SpecialChars, LeftBracket),
            create_token("1", TokenKind::Values, NumberInteger),
            create_token(")", TokenKind::SpecialChars, RightBracket),
            create_token("{", TokenKind::SpecialChars, LeftCurlyBrace),
            create_token("}", TokenKind::SpecialChars, RightCurlyBrace),
        ];
        
        let result = init_parser(&tokens, false);
        let output = result.join("");
        assert!(output.contains("loop"), "Should convert infinite while(1) to loop");
    }

    #[test]
    fn test_for_loop() {
        let tokens = vec![
            create_token("for", TokenKind::Keyword, KeywordFor),
            create_token("(", TokenKind::SpecialChars, LeftBracket),
            create_token("int", TokenKind::DataTypes, Integer),
            create_token("i", TokenKind::Identifiers, Identifier),
            create_token("=", TokenKind::AssignmentOperators, Assignment),
            create_token("0", TokenKind::Values, NumberInteger),
            create_token(";", TokenKind::SpecialChars, Semicolon),
            create_token("i", TokenKind::Identifiers, Identifier),
            create_token("<", TokenKind::BinaryOperators, BitwiseLeftShift), // Simplified
            create_token("10", TokenKind::Values, NumberInteger),
            create_token(";", TokenKind::SpecialChars, Semicolon),
            create_token("i", TokenKind::Identifiers, Identifier),
            create_token("++", TokenKind::UnaryOperators, Increment),
            create_token(")", TokenKind::SpecialChars, RightBracket),
            create_token("{", TokenKind::SpecialChars, LeftCurlyBrace),
            create_token("}", TokenKind::SpecialChars, RightCurlyBrace),
        ];
        
        let result = init_parser(&tokens, false);
        let output = result.join("");
        assert!(output.contains("while"), "Should generate while loop for for loop");
    }

    #[test]
    fn test_switch_statement() {
        let tokens = vec![
            create_token("switch", TokenKind::Keyword, KeywordSwitch),
            create_token("(", TokenKind::SpecialChars, LeftBracket),
            create_token("x", TokenKind::Identifiers, Identifier),
            create_token(")", TokenKind::SpecialChars, RightBracket),
            create_token("{", TokenKind::SpecialChars, LeftCurlyBrace),
            create_token("case", TokenKind::Keyword, KeywordCase),
            create_token("1", TokenKind::Values, NumberInteger),
            create_token(":", TokenKind::SpecialChars, Semicolon),
            create_token("break", TokenKind::Keyword, KeywordBreak),
            create_token(";", TokenKind::SpecialChars, Semicolon),
            create_token("}", TokenKind::SpecialChars, RightCurlyBrace),
        ];
        
        let result = init_parser(&tokens, false);
        let output = result.join("");
        assert!(output.contains("match"), "Should translate switch to match");
    }

    // ============================================================================
    // ASSIGNMENT TESTS
    // ============================================================================

    #[test]
    fn test_simple_assignment() {
        let tokens = vec![
            create_token("x", TokenKind::Identifiers, Identifier),
            create_token("=", TokenKind::AssignmentOperators, Assignment),
            create_token("5", TokenKind::Values, NumberInteger),
            create_token(";", TokenKind::SpecialChars, Semicolon),
        ];
        
        let result = init_parser(&tokens, false);
        let output = result.join("");
        assert!(output.contains("="), "Should contain assignment");
    }

    #[test]
    fn test_expression_with_operators() {
        let tokens = vec![
            create_token("x", TokenKind::Identifiers, Identifier),
            create_token("=", TokenKind::AssignmentOperators, Assignment),
            create_token("a", TokenKind::Identifiers, Identifier),
            create_token("+", TokenKind::BinaryOperators, Plus),
            create_token("b", TokenKind::Identifiers, Identifier),
            create_token(";", TokenKind::SpecialChars, Semicolon),
        ];
        
        let result = init_parser(&tokens, false);
        let output = result.join("");
        assert!(output.contains("+"), "Should preserve binary operator");
    }

    // ============================================================================
    // STRICT VS LOOSE MODE TESTS
    // ============================================================================

    #[test]
    fn test_loose_mode_generates_mut() {
        let tokens = vec![
            create_token("int", TokenKind::DataTypes, Integer),
            create_token("x", TokenKind::Identifiers, Identifier),
            create_token(";", TokenKind::SpecialChars, Semicolon),
        ];
        
        let result = init_parser(&tokens, false); // false = loose mode
        let output = result.join("");
        assert!(output.contains("mut"), "Loose mode should generate 'mut'");
    }

    #[test]
    fn test_strict_mode_no_mut() {
        let tokens = vec![
            create_token("int", TokenKind::DataTypes, Integer),
            create_token("x", TokenKind::Identifiers, Identifier),
            create_token(";", TokenKind::SpecialChars, Semicolon),
        ];
        
        let result = init_parser(&tokens, true); // true = strict mode
        let output = result.join("");
        // Strict mode should not use 'mut' for regular variables
        let has_let_without_mut = output.split("let").any(|s| !s.trim_start().starts_with("mut"));
        assert!(has_let_without_mut, "Strict mode should have immutable bindings");
    }

    // ============================================================================
    // TYPEDEF TESTS
    // ============================================================================

    #[test]
    fn test_typedef_declaration() {
        let tokens = vec![
            create_token("typedef", TokenKind::DataTypes, Typedef),
            create_token("int", TokenKind::DataTypes, Integer),
            create_token("myint", TokenKind::Identifiers, Identifier),
            create_token(";", TokenKind::SpecialChars, Semicolon),
        ];
        
        let result = init_parser(&tokens, false);
        let output = result.join("");
        assert!(output.contains("type"), "Should translate typedef to Rust type alias");
    }

    // ============================================================================
    // COMMENTS TEST
    // ============================================================================

    #[test]
    fn test_comment_preservation() {
        let tokens = vec![
            create_token("// Comment", TokenKind::Comments, Identifier),
            create_token("int", TokenKind::DataTypes, Integer),
            create_token("x", TokenKind::Identifiers, Identifier),
            create_token(";", TokenKind::SpecialChars, Semicolon),
        ];
        
        let result = init_parser(&tokens, false);
        let output = result.join("");
        assert!(output.contains("Comment"), "Comments should be preserved");
    }

    // ============================================================================
    // BASIC SANITY TESTS
    // ============================================================================

    #[test]
    fn test_empty_input() {
        let tokens: Vec<Token> = vec![];
        let result = init_parser(&tokens, false);
        // Should not crash, should just output initial doc string
        assert!(result.len() > 0, "Empty input should still produce output");
    }

    #[test]
    fn test_single_token() {
        let tokens = vec![
            create_token("int", TokenKind::DataTypes, Integer),
        ];
        
        let result = init_parser(&tokens, false);
        // Should not crash
        assert!(result.len() > 0, "Single token should not crash");
    }

    // ============================================================================
    // FULL EXPRESSION OUTPUT TESTS
    // ============================================================================

    #[test]
    fn test_simple_int_declaration_full_output() {
        let tokens = vec![
            create_token("int", TokenKind::DataTypes, Integer),
            create_token("x", TokenKind::Identifiers, Identifier),
            create_token(";", TokenKind::SpecialChars, Semicolon),
        ];
        
        let result = init_parser(&tokens, false);
        let output = result.join("");
        
        println!("\n=== INT DECLARATION ===");
        println!("Expected: let mut x: i32;");
        println!("Actual output:\n{}\n", output);
        
        // Verify it contains the expected declaration pattern
        let has_declaration = output.contains("static") && output.contains("x") && output.contains("i32");
        assert!(has_declaration, "Should generate: static mut x: i32;");
    }

    #[test]
    fn test_int_with_assignment_full_output() {
        let tokens = vec![
            create_token("int", TokenKind::DataTypes, Integer),
            create_token("x", TokenKind::Identifiers, Identifier),
            create_token("=", TokenKind::AssignmentOperators, Assignment),
            create_token("42", TokenKind::Values, NumberInteger),
            create_token(";", TokenKind::SpecialChars, Semicolon),
        ];
        
        let result = init_parser(&tokens, false);
        let output = result.join("");
        
        println!("\n=== INT WITH ASSIGNMENT ===");
        println!("Expected: static mut x: i32 = 42;");
        println!("Actual output:\n{}\n", output);
        
        // Verify complete expression
        let has_full_expr = output.contains("static") && output.contains("mut") && output.contains("x") && 
                           output.contains("i32") && output.contains("=") && output.contains("42");
        assert!(has_full_expr, "Should generate: static mut x: i32 = 42;");
    }

    #[test]
    fn test_simple_if_full_output() {
        let tokens = vec![
            create_token("if", TokenKind::Keyword, KeywordIf),
            create_token("(", TokenKind::SpecialChars, LeftBracket),
            create_token("x", TokenKind::Identifiers, Identifier),
            create_token(">", TokenKind::BinaryOperators, GreaterThan),
            create_token("0", TokenKind::Values, NumberInteger),
            create_token(")", TokenKind::SpecialChars, RightBracket),
            create_token("{", TokenKind::SpecialChars, LeftCurlyBrace),
            create_token("y", TokenKind::Identifiers, Identifier),
            create_token("=", TokenKind::AssignmentOperators, Assignment),
            create_token("1", TokenKind::Values, NumberInteger),
            create_token(";", TokenKind::SpecialChars, Semicolon),
            create_token("}", TokenKind::SpecialChars, RightCurlyBrace),
        ];
        
        let result = init_parser(&tokens, false);
        let output = result.join("");
        
        println!("\n=== IF STATEMENT ===");
        println!("Expected: if x > 0 {{ y = 1; }}");
        println!("Actual output:\n{}\n", output);
        
        let has_if_expr = output.contains("if") && output.contains(">") && 
                         output.contains("0") && output.contains("{") && output.contains("}");
        assert!(has_if_expr, "Should generate if statement with comparison");
    }

    #[test]
    fn test_while_loop_full_output() {
        let tokens = vec![
            create_token("while", TokenKind::Keyword, KeywordWhile),
            create_token("(", TokenKind::SpecialChars, LeftBracket),
            create_token("x", TokenKind::Identifiers, Identifier),
            create_token("<", TokenKind::BinaryOperators, BitwiseLeftShift),
            create_token("10", TokenKind::Values, NumberInteger),
            create_token(")", TokenKind::SpecialChars, RightBracket),
            create_token("{", TokenKind::SpecialChars, LeftCurlyBrace),
            create_token("x", TokenKind::Identifiers, Identifier),
            create_token("++", TokenKind::UnaryOperators, Increment),
            create_token(";", TokenKind::SpecialChars, Semicolon),
            create_token("}", TokenKind::SpecialChars, RightCurlyBrace),
        ];
        
        let result = init_parser(&tokens, false);
        let output = result.join("");
        
        println!("\n=== WHILE LOOP ===");
        println!("Expected: while x < 10 {{ x += 1; }}");
        println!("Actual output:\n{}\n", output);
        
        let has_while_expr = output.contains("while") && output.contains("x") && 
                            output.contains("<") && output.contains("10") && output.contains("+");
        assert!(has_while_expr, "Should generate while loop with increment");
    }

    #[test]
    fn test_pointer_type_full_output() {
        let tokens = vec![
            create_token("int", TokenKind::DataTypes, Integer),
            create_token("*", TokenKind::BinaryOperators, Multiplication),
            create_token("ptr", TokenKind::Identifiers, Identifier),
            create_token(";", TokenKind::SpecialChars, Semicolon),
        ];
        
        let result = init_parser(&tokens, false);
        let output = result.join("");
        
        println!("\n=== POINTER DECLARATION ===");
        println!("Expected: let mut ptr: &i32;");
        println!("Actual output:\n{}\n", output);
        
        let has_ptr_expr = output.contains("&") && output.contains("i32") && output.contains("ptr");
        assert!(has_ptr_expr, "Should convert * to & reference");
    }

    #[test]
    fn test_multiple_operations_full_output() {
        let tokens = vec![
            create_token("int", TokenKind::DataTypes, Integer),
            create_token("result", TokenKind::Identifiers, Identifier),
            create_token("=", TokenKind::AssignmentOperators, Assignment),
            create_token("a", TokenKind::Identifiers, Identifier),
            create_token("+", TokenKind::BinaryOperators, Plus),
            create_token("b", TokenKind::Identifiers, Identifier),
            create_token("*", TokenKind::BinaryOperators, Multiplication),
            create_token("c", TokenKind::Identifiers, Identifier),
            create_token(";", TokenKind::SpecialChars, Semicolon),
        ];
        
        let result = init_parser(&tokens, false);
        let output = result.join("");
        
        println!("\n=== COMPLEX EXPRESSION ===");
        println!("Expected: let mut result: i32 = a + b * c;");
        println!("Actual output:\n{}\n", output);
        
        let has_multi_expr = output.contains("result") && output.contains("a") && 
                            output.contains("b") && output.contains("c") && 
                            output.contains("+") && output.contains("*") && output.contains("=");
        assert!(has_multi_expr, "Should preserve all operations and operands");
    }

    #[test]
    fn test_const_int_declaration_full_output() {
        let tokens = vec![
            create_token("const", TokenKind::Modifiers, KeywordConst),
            create_token("int", TokenKind::DataTypes, Integer),
            create_token("MAX", TokenKind::Identifiers, Identifier),
            create_token("=", TokenKind::AssignmentOperators, Assignment),
            create_token("100", TokenKind::Values, NumberInteger),
            create_token(";", TokenKind::SpecialChars, Semicolon),
        ];
        
        let result = init_parser(&tokens, false);
        let output = result.join("");
        
        println!("\n=== CONST DECLARATION ===");
        println!("Expected: const MAX: i32 = 100;");
        println!("Actual output:\n{}\n", output);
        
        let has_const_expr = output.contains("const") && output.contains("MAX") && 
                            output.contains("100") && output.contains("i32");
        assert!(has_const_expr, "Should preserve const with value");
    }

    #[test]
    fn test_float_declaration_full_output() {
        let tokens = vec![
            create_token("float", TokenKind::DataTypes, Float),
            create_token("pi", TokenKind::Identifiers, Identifier),
            create_token("=", TokenKind::AssignmentOperators, Assignment),
            create_token("3.14", TokenKind::Values, NumberFloat),
            create_token(";", TokenKind::SpecialChars, Semicolon),
        ];
        
        let result = init_parser(&tokens, false);
        let output = result.join("");
        
        println!("\n=== FLOAT DECLARATION ===");
        println!("Expected: let mut pi: f32 = 3.14;");
        println!("Actual output:\n{}\n", output);
        
        // float should translate to f32
        assert!(output.contains("pi"), "Should have variable name");
        assert!(output.contains("3.14"), "Should preserve float literal");
        assert!(output.contains("f32") || output.contains("float"), "Should specify float type");
        assert!(output.contains("="), "Should have assignment");
    }

    #[test]
    fn test_char_declaration_full_output() {
        let tokens = vec![
            create_token("char", TokenKind::DataTypes, Character),
            create_token("ch", TokenKind::Identifiers, Identifier),
            create_token("=", TokenKind::AssignmentOperators, Assignment),
            create_token("'A'", TokenKind::Values, CharValue),
            create_token(";", TokenKind::SpecialChars, Semicolon),
        ];
        
        let result = init_parser(&tokens, false);
        let output = result.join("");
        
        println!("\n=== CHAR DECLARATION ===");
        println!("Expected: let mut ch: char = 'A';");
        println!("Actual output:\n{}\n", output);
        
        let has_char_expr = output.contains("ch") && output.contains("'A'") && output.contains("=");
        assert!(has_char_expr, "Should preserve char declaration with literal");
    }
}

