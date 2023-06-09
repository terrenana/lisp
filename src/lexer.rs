#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Integer(i64),
    Symbol(String),
    OpenParen,
    CloseParen,
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Integer(n) => write!(f, "{}", n),
            Token::Symbol(s) => write!(f, "{}", s),
            Token::OpenParen => write!(f, "("),
            Token::CloseParen => write!(f, ")"),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Symbol {
    Add,
    Sub,
    Mul,
    Div,
}

impl std::fmt::Display for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Symbol::Add => write!(f, "+"),
            Symbol::Sub => write!(f, "-"),
            Symbol::Mul => write!(f, "*"),
            Symbol::Div => write!(f, "/"),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Keyword {
    Def,
    Set,
}

impl std::fmt::Display for Keyword {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Keyword::Def => write!(f, "def"),
            Keyword::Set => write!(f, "set"),
        }
    }
}

#[derive(Debug)]
pub struct TokenError {
    ch: char,
}

impl std::fmt::Display for TokenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Unexpected character: {}", self.ch)
    }
}

pub fn tokenize(stream: &str) -> Result<Vec<Token>, TokenError> {
    let program = stream.replace(")", " ) ").replace("(", " ( ");
    let words = program.split_whitespace();

    let mut tokens = Vec::new();

    for word in words {
        match word {
            "(" => tokens.push(Token::OpenParen),
            ")" => tokens.push(Token::CloseParen),
            _ => tokens.push(lex_string(word)),
        }
    }

    Ok(tokens)
}

fn lex_string(string: &str) -> Token {
    if let Ok(i) = string.parse::<i64>() {
        Token::Integer(i)
    } else {
        Token::Symbol(string.to_string())
    }
}
