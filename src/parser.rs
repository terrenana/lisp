use crate::{
    lexer::{tokenize, Token, TokenError},
    object::Object,
};

#[derive(Debug)]
pub struct ParseError {
    err: String,
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.err)
    }
}

pub fn parse(stream: &str) -> Result<Object, ParseError> {
    if let Ok(tokens) = tokenize(stream) {
        let mut tokens = tokens.into_iter().rev().collect::<Vec<_>>();
        let obj_list = parse_list(&mut tokens)?;
        Ok(obj_list)
    } else {
        Err(ParseError {
            err: format!("{}", tokenize(stream).err().unwrap()),
        })
    }
}

fn parse_list(tokens: &mut Vec<Token>) -> Result<Object, ParseError> {
    let token = tokens.pop();
    if token != Some(Token::OpenParen) {
        return Err(ParseError {
            err: format!("Expected OpenParen, found {:#?}", token),
        });
    }

    let mut list: Vec<Object> = Vec::new();

    loop {
        if let Some(token) = tokens.pop() {
            match token {
                Token::Integer(n) => list.push(Object::Integer(n)),
                Token::Symbol(s) => list.push(Object::Symbol(s)),
                // Token::Keyword(k) => list.push(Object::Keyword(k)),
                // Token::Ident(i) => list.push(Object::Ident(i)),
                Token::OpenParen => {
                    tokens.push(Token::OpenParen);
                    let sub_list = parse_list(tokens)?;
                    list.push(sub_list);
                }
                Token::CloseParen => return Ok(Object::List(list)),
            }
        } else {
            return Err(ParseError {
                err: format!("Insufficient Tokens"),
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let list = parse("(+ 1 2 )").unwrap();
    }
}
