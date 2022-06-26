#[derive(Debug)]
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

pub fn tokenize(text: String) -> Vec<TokenKind> {
    let mut text = text.clone();
    let mut tokens = Vec::new();

    let mut token = get_next_token(&mut text);
    while token.is_some() {
        tokens.push(token.unwrap());
        token = get_next_token(&mut text);
    }

    if !text.is_empty() {

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

    let token = match ch {
        None => None,
        Some(ch) => {
            match ch {
                ';' => Some(TokenKind::Semicolon),
                '(' => Some(TokenKind::OpenParenthesis),
                ')' => Some(TokenKind::CloseParenthesis),
                '{' => Some(TokenKind::OpenBrace),
                '}' => Some(TokenKind::CloseBrace),
                _ => None
            }
        }
    };

    return match token {
        Some(token) => Some((token, 1)),
        None => None
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
