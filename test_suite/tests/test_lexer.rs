use libastral::lexer::tokenize;

#[test]
fn foo() {
    assert_eq!(tokenize(""), vec![]);
}
