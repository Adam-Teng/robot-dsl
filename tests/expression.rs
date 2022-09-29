use robot_dsl::{interpreter::Interpreter, parser::Parser, scanner::Scanner};

#[test]
fn test_interpreter_cal_binary() {
    let mut scanner = Scanner::new("1 + 2 + 3".to_string());
    let tokens = scanner.scan_tokens();
    let mut parser = Parser::new(&tokens);
    if let Some(expression) = parser.calculate() {
        let mut interpreter = Interpreter::new();
        assert!(interpreter.interpret_cal(&expression).is_ok());
        let res: String = interpreter.interpret_cal(&expression).unwrap().to_string();
        assert_eq!(res, "6");
    }
}

#[test]
fn test_interpreter_cal_unary_true() {
    let mut scanner = Scanner::new("!1".to_string());
    let tokens = scanner.scan_tokens();
    let mut parser = Parser::new(&tokens);
    if let Some(expression) = parser.calculate() {
        let mut interpreter = Interpreter::new();
        assert!(interpreter.interpret_cal(&expression).is_ok());
        let res: String = interpreter.interpret_cal(&expression).unwrap().to_string();
        assert_eq!(res, "false");
    }
}

#[test]
fn test_interpreter_cal_unary_false() {
    let mut scanner = Scanner::new("!0".to_string());
    let tokens = scanner.scan_tokens();
    let mut parser = Parser::new(&tokens);
    if let Some(expression) = parser.calculate() {
        let mut interpreter = Interpreter::new();
        assert!(interpreter.interpret_cal(&expression).is_ok());
        let res: String = interpreter.interpret_cal(&expression).unwrap().to_string();
        assert_eq!(res, "false");
    }
}

#[test]
fn test_interpreter_binary_equal() {
    let mut scanner = Scanner::new("1 == 1".to_string());
    let tokens = scanner.scan_tokens();
    let mut parser = Parser::new(&tokens);
    if let Some(expression) = parser.calculate() {
        let mut interpreter = Interpreter::new();
        assert!(interpreter.interpret_cal(&expression).is_ok());
        let res: String = interpreter.interpret_cal(&expression).unwrap().to_string();
        assert_eq!(res, "true");
    }
}

#[test]
fn test_interpreter_binary_not_equal() {
    let mut scanner = Scanner::new("1 != 1".to_string());
    let tokens = scanner.scan_tokens();
    let mut parser = Parser::new(&tokens);
    if let Some(expression) = parser.calculate() {
        let mut interpreter = Interpreter::new();
        assert!(interpreter.interpret_cal(&expression).is_ok());
        let res: String = interpreter.interpret_cal(&expression).unwrap().to_string();
        assert_eq!(res, "false");
    }
}

#[test]
fn test_interpreter_binary_plus1() {
    let mut scanner = Scanner::new("1 + 1".to_string());
    let tokens = scanner.scan_tokens();
    let mut parser = Parser::new(&tokens);
    if let Some(expression) = parser.calculate() {
        let mut interpreter = Interpreter::new();
        assert!(interpreter.interpret_cal(&expression).is_ok());
        let res: String = interpreter.interpret_cal(&expression).unwrap().to_string();
        assert_eq!(res, "2");
    }
}

#[test]
fn test_interpreter_binary_plus2() {
    let mut scanner = Scanner::new("1 + 1 + 1".to_string());
    let tokens = scanner.scan_tokens();
    let mut parser = Parser::new(&tokens);
    if let Some(expression) = parser.calculate() {
        let mut interpreter = Interpreter::new();
        assert!(interpreter.interpret_cal(&expression).is_ok());
        let res: String = interpreter.interpret_cal(&expression).unwrap().to_string();
        assert_eq!(res, "3");
    }
}

#[test]
fn test_interpreter_binary_minus1() {
    let mut scanner = Scanner::new("1 - 1".to_string());
    let tokens = scanner.scan_tokens();
    let mut parser = Parser::new(&tokens);
    if let Some(expression) = parser.calculate() {
        let mut interpreter = Interpreter::new();
        assert!(interpreter.interpret_cal(&expression).is_ok());
        let res: String = interpreter.interpret_cal(&expression).unwrap().to_string();
        assert_eq!(res, "0");
    }
}

#[test]
fn test_interpreter_binary_minus2() {
    let mut scanner = Scanner::new("1 - 1 - 1".to_string());
    let tokens = scanner.scan_tokens();
    let mut parser = Parser::new(&tokens);
    if let Some(expression) = parser.calculate() {
        let mut interpreter = Interpreter::new();
        assert!(interpreter.interpret_cal(&expression).is_ok());
        let res: String = interpreter.interpret_cal(&expression).unwrap().to_string();
        assert_eq!(res, "-1");
    }
}

#[test]
fn test_interperter_call_primary_number() {
    let mut scanner = Scanner::new("1".to_string());
    let tokens = scanner.scan_tokens();
    let mut parser = Parser::new(&tokens);
    if let Some(expression) = parser.calculate() {
        let mut interpreter = Interpreter::new();
        assert!(interpreter.interpret_cal(&expression).is_ok());
        let res: String = interpreter.interpret_cal(&expression).unwrap().to_string();
        assert_eq!(res, "1");
    }
}

#[test]
fn test_interpreter_call_primary_string() {
    let mut scanner = Scanner::new("\"hello\"".to_string());
    let tokens = scanner.scan_tokens();
    let mut parser = Parser::new(&tokens);
    if let Some(expression) = parser.calculate() {
        let mut interpreter = Interpreter::new();
        assert!(interpreter.interpret_cal(&expression).is_ok());
        let res: String = interpreter.interpret_cal(&expression).unwrap().to_string();
        assert_eq!(res, "hello");
    }
}

#[test]
fn test_interpreter_call_primary_true() {
    let mut scanner = Scanner::new("true".to_string());
    let tokens = scanner.scan_tokens();
    let mut parser = Parser::new(&tokens);
    if let Some(expression) = parser.calculate() {
        let mut interpreter = Interpreter::new();
        assert!(interpreter.interpret_cal(&expression).is_ok());
        let res: String = interpreter.interpret_cal(&expression).unwrap().to_string();
        assert_eq!(res, "true");
    }
}

#[test]
fn test_interpreter_call_primary_false() {
    let mut scanner = Scanner::new("false".to_string());
    let tokens = scanner.scan_tokens();
    let mut parser = Parser::new(&tokens);
    if let Some(expression) = parser.calculate() {
        let mut interpreter = Interpreter::new();
        assert!(interpreter.interpret_cal(&expression).is_ok());
        let res: String = interpreter.interpret_cal(&expression).unwrap().to_string();
        assert_eq!(res, "false");
    }
}

