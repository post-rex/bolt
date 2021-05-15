use crate::config::parser::tokenizer::{tokenize, Token::*};

#[test]
fn single() {
    assert_eq!(tokenize("statement"), Ok(vec![Statement("statement"), Eof]));
}

#[test]
fn combined_space() {
    assert_eq!(
        tokenize("hello world"),
        Ok(vec![Statement("hello"), Spacer, Statement("world"), Eof])
    );
}

#[test]
fn combined_dot() {
    assert_eq!(
        tokenize("tls.session"),
        Ok(vec![Statement("tls"), Dot, Statement("session"), Eof])
    );
}

#[test]
fn combined_newline() {
    assert_eq!(
        tokenize("hi\nyou"),
        Ok(vec![Statement("hi"), NewLine, Statement("you"), Eof])
    );
}

#[test]
fn numbers() {
    assert_eq!(
        tokenize("hi m8\n"),
        Ok(vec![Statement("hi"), Spacer, Statement("m8"), NewLine, Eof])
    )
}