use crate::lexer::{Keyword, Symbol};

#[derive(Debug, PartialEq, Clone)]
pub enum Object {
    Void,
    Integer(i64),
    Bool(bool),
    Symbol(String),
    // Keyword(Keyword),
    // Ident(String),
    Lambda(Vec<String>, Vec<Object>),
    List(Vec<Object>),
}
