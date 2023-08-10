#![allow(unused)]
use super::*;
use std::error::Error;
#[test]
fn TestLetStatements() {
    let input = r#"
        let x = 5;
        let y = 10;
        let foobar = 838383;
        "#;
    let mut l = Lexer::new(input.to_string());
    let mut parser = Parser::new(&mut l);

    let program = parser.parse_program();
    assert_eq!(
        program.statements.len(),
        3,
        "amount of statement mismatched"
    );
    let expected_ident = ["x", "y", "foobar"];
    for ind in 0..3 {
        match TestSingleLetStatement() {
            Ok(_) => {}
            _ => assert!(false, "statement {ind} parse error"),
        }
    }
    assert!(true);
}

fn TestSingleLetStatement() -> Result<(), Box<dyn Error>> {
    Ok(())
}
