use lazy_static::lazy_static;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum TokenKind {
    OpenBrace,
    CloseBrace,
    OpenParenthesis,
    CloseParenthesis,
    Semicolon,
    IntKeyword,
    ReturnKeyword,
    Identifier(String),
    IntegerLiteral(i32),
}

const WHITESPACES: [char; 2] = [' ', '\t'];

lazy_static! {

    static ref BASIC_TOKENS: HashMap<char, TokenKind> = {
        HashMap::from([
            (';', TokenKind::Semicolon),
            ('(', TokenKind::OpenParenthesis),
            (')', TokenKind::CloseParenthesis),
            ('{', TokenKind::OpenBrace),
            ('}', TokenKind::CloseBrace),
        ])
    };

    static ref KEYWORD_TOKENS: HashMap<&'static str, TokenKind> = {
        HashMap::from([
            ("return", TokenKind::ReturnKeyword)
        ])
    };

}

pub fn tokenize(text: String) -> Vec<TokenKind> {
    let mut text = text.clone();
    let mut tokens = Vec::new();

    let mut token = get_next_token(&mut text);
    while token.is_some() {
        tokens.push(token.unwrap());
        token = get_next_token(&mut text);
    }

    if !text.is_empty() {
        // TODO: Throw exception or something
    }

    return tokens
}

fn get_next_token(text: &mut String) -> Option<TokenKind> {
    skip_whitespaces(text);

    let tokenize_fns = [
        tokenize_basic,
        tokenize_keyword,
        tokenize_identifier,
        tokenize_value
    ];

    for tokenize_fn in tokenize_fns {
        let token_and_size = tokenize_fn(text);
        if token_and_size.is_some() {
            let (token, size) = token_and_size.unwrap();
            remove_left(text, size);
            return Some(token);
        }
    };

    return None;
}

fn is_whitespace(c: char) -> bool {
    return WHITESPACES.contains(&c);
}

fn skip_whitespaces(text: &mut String) {
    loop {
        let oc = text.chars().nth(0);
        match oc {
            None => {
                break;
            }
            Some(c) => {
                if is_whitespace(c) {
                    text.remove(0);
                }
                else {
                    break;
                }
            }
        }
    }
}

fn remove_left(text: &mut String, size: usize) {
    text.replace_range(..size, "");
}

fn tokenize_basic(text: &String) -> Option<(TokenKind, usize)> {
    let ch = text.chars().nth(0);

    return match ch {
        None => None,
        Some(ch) => {
            for (token_ch, token) in BASIC_TOKENS.iter() {
                if ch == *token_ch {
                    return Some((token.clone(), 1));
                }
            }
            None
        }
    };
}

fn tokenize_keyword(text: &String) -> Option<(TokenKind, usize)> {
    return None
}

fn tokenize_identifier(text: &String) -> Option<(TokenKind, usize)> {
    return None
}

fn tokenize_value(text: &String) -> Option<(TokenKind, usize)> {
    return None
}
