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
