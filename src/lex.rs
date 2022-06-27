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

const WHITESPACES: [char; 3] = [' ', '\t', '\n'];

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
            ("return", TokenKind::ReturnKeyword),
            ("int", TokenKind::IntKeyword),
        ])
    };

}

pub fn lex(text: String) -> Vec<TokenKind> {
    let raw_tokens = split_tokens(&text);
    let mut tokens = recognize_tokens(&raw_tokens);
    return tokens
}

fn split_tokens(text: &String) -> Vec<String> {
    let mut tokens: Vec<String> = Vec::new();
    let mut token = String::new();

    for c in text.chars() {
        let white = is_whitespace(&c);
        let basic = is_basic_char(&c);

        if white || basic {
            if !token.is_empty() {
                tokens.push(token.clone());
                token.clear();
            }
            if basic {
                tokens.push(String::from(c));
            }
        }
        else {
            token.push(c);
        }
    }

    return tokens;
}

fn recognize_tokens(raw_tokens: &Vec<String>) -> Vec<TokenKind> {
    let mut tokens: Vec<TokenKind> = Vec::new();
    for raw_token in raw_tokens {
        let token = recognize_token(raw_token);
        if token.is_some() {
            let token = token.unwrap();
            tokens.push(token);
        }
    }
    return tokens
}

fn recognize_token(raw_token: &String) -> Option<TokenKind> {

    // return token if basic
    let basic = recognize_basic_token(raw_token);
    if basic.is_some() {
        return basic;
    }

    // return token if keyword
    let keyword = recognize_keyword_token(raw_token);
    if keyword.is_some() {
        return keyword;
    }

    // return token if identifier
    let identifier = recognize_identifier_token(raw_token);
    if identifier.is_some() {
        return identifier;
    }

    // return token if literal
    let literal = recognize_literal_token(raw_token);
    if literal.is_some() {
        return literal;
    }

    return None;
}

fn recognize_basic_token(raw_token: &String) -> Option<TokenKind> {
    if raw_token.len() > 1 {
        return None;
    }

    let c = raw_token.chars().nth(0).unwrap();
    let token = BASIC_TOKENS.get(&c);
    return match token {
        Some(token) => Some(token.clone()),
        None => None
    }
}

fn recognize_keyword_token(raw_token: &String) -> Option<TokenKind> {
    let token = KEYWORD_TOKENS.get(raw_token.as_str());
    return match token {
        Some(token) => Some(token.clone()),
        None => None
    }
}

fn recognize_identifier_token(raw_token: &String) -> Option<TokenKind> {
    let start_char = &raw_token.chars().nth(0).unwrap();
    return if is_valid_identifier_start_char(start_char) {
        Some(TokenKind::Identifier(raw_token.clone()))
    } else {
        None
    }
}

fn is_valid_identifier_start_char(c: &char) -> bool {
    return c.is_ascii_alphabetic() || *c == '_';
}

fn recognize_literal_token(raw_token: &String) -> Option<TokenKind> {
    if is_integer_value(raw_token) {
        let num: i32 = raw_token.parse().unwrap();
        return Some(TokenKind::IntegerLiteral(num));
    }
    else {
        return None;
    }
}

fn is_integer_value(raw_token: &String) -> bool {
    let start_char = &raw_token.chars().nth(0).unwrap();
    return start_char.is_digit(10);
}

fn is_whitespace(c: &char) -> bool {
    return WHITESPACES.contains(&c);
}

fn is_basic_char(c: &char) -> bool {
    return BASIC_TOKENS.contains_key(&c);
}
