use robot_dsl::{interpreter::Interpreter, parser::Parser, scanner::Scanner};

#[test]
fn test_interpreter_speak_string() {
    let speaking: String = "speak \"hello\";".to_string();
    let mut scanner = Scanner::new(speaking);
    let tokens = scanner.scan_tokens();
    let mut parser = Parser::new(&tokens);
    let statements = parser.parse().unwrap();
    let mut interpreter = Interpreter::new();
    assert!(interpreter.interpret(&statements).is_ok());
}

#[test]
fn test_interpreter_speak_number() {
    let speaking: String = "speak 1;".to_string();
    let mut scanner = Scanner::new(speaking);
    let tokens = scanner.scan_tokens();
    let mut parser = Parser::new(&tokens);
    let statements = parser.parse().unwrap();
    let mut interpreter = Interpreter::new();
    assert!(interpreter.interpret(&statements).is_ok());
}

#[test]
fn test_interpreter_speak_true() {
    let speaking: String = "speak true;".to_string();
    let mut scanner = Scanner::new(speaking);
    let tokens = scanner.scan_tokens();
    let mut parser = Parser::new(&tokens);
    let statements = parser.parse().unwrap();
    let mut interpreter = Interpreter::new();
    assert!(interpreter.interpret(&statements).is_ok());
}

#[test]
fn test_interpreter_speak_false() {
    let speaking: String = "speak false;".to_string();
    let mut scanner = Scanner::new(speaking);
    let tokens = scanner.scan_tokens();
    let mut parser = Parser::new(&tokens);
    let statements = parser.parse().unwrap();
    let mut interpreter = Interpreter::new();
    assert!(interpreter.interpret(&statements).is_ok());
}

#[test]
fn test_interpreter_speak_expression() {
    let speaking: String = "speak 1 + 1;".to_string();
    let mut scanner = Scanner::new(speaking);
    let tokens = scanner.scan_tokens();
    let mut parser = Parser::new(&tokens);
    let statements = parser.parse().unwrap();
    let mut interpreter = Interpreter::new();
    assert!(interpreter.interpret(&statements).is_ok());
}

#[test]
fn test_interpreter_var_expression() {
    let speaking: String = "var a = 1 + 1; speak a;".to_string();
    let mut scanner = Scanner::new(speaking);
    let tokens = scanner.scan_tokens();
    let mut parser = Parser::new(&tokens);
    let statements = parser.parse().unwrap();
    let mut interpreter = Interpreter::new();
    assert!(interpreter.interpret(&statements).is_ok());
}

#[test]
fn test_interpreter_var_expression_with_assignment() {
    let speaking: String = "var a = 1 + 1; speak a; a = 2; speak a;".to_string();
    let mut scanner = Scanner::new(speaking);
    let tokens = scanner.scan_tokens();
    let mut parser = Parser::new(&tokens);
    let statements = parser.parse().unwrap();
    let mut interpreter = Interpreter::new();
    assert!(interpreter.interpret(&statements).is_ok());
}

#[test]
fn test_interpreter_var_expression_with_assignment_and_reassignment() {
    let speaking: String = "var a = 1 + 1; speak a; a = 2; speak a; a = 3; speak a;".to_string();
    let mut scanner = Scanner::new(speaking);
    let tokens = scanner.scan_tokens();
    let mut parser = Parser::new(&tokens);
    let statements = parser.parse().unwrap();
    let mut interpreter = Interpreter::new();
    assert!(interpreter.interpret(&statements).is_ok());
}

#[test]
fn test_interpreter_scope() {
    let speaking: String = "var a = 1 + 1; speak a; { var a = 2; speak a; } speak a;".to_string();
    let mut scanner = Scanner::new(speaking);
    let tokens = scanner.scan_tokens();
    let mut parser = Parser::new(&tokens);
    let statements = parser.parse().unwrap();
    let mut interpreter = Interpreter::new();
    assert!(interpreter.interpret(&statements).is_ok());
}

#[test]
fn test_interpreter_scope_with_reassignment() {
    let speaking: String = "var a = 1 + 1; speak a; { var a = 2; speak a; a = 3; speak a; } speak a;".to_string();
    let mut scanner = Scanner::new(speaking);
    let tokens = scanner.scan_tokens();
    let mut parser = Parser::new(&tokens);
    let statements = parser.parse().unwrap();
    let mut interpreter = Interpreter::new();
    assert!(interpreter.interpret(&statements).is_ok());
}

#[test]
fn test_interpreter_branch_statement() {
    let speaking: String = "branch (true) { speak 1; }".to_string();
    let mut scanner = Scanner::new(speaking);
    let tokens = scanner.scan_tokens();
    let mut parser = Parser::new(&tokens);
    let statements = parser.parse().unwrap();
    let mut interpreter = Interpreter::new();
    assert!(interpreter.interpret(&statements).is_ok());
}
