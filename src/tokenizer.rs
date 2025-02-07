use std::collections::{HashMap, HashSet};

use crate::IvyError;



pub enum Token {
    Tabs(usize),
    Spaces(usize),
    GenericWhite,

    Parentheses(Vec<Token>),
    Brackets(Vec<Token>),
    Braces(Vec<Token>),
    String(String),
    Variable(String),

    Bool(bool),
    Char(char),

    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Eq,
    Neq,
    Lt,
    Gt,
    Lte,
    Gte,
    Not,

    Equal,

    Let,
    Var,
    Colon,
    Comma,
    Dollar,
    Underscore,


}



//pub fn tokenize(input: &Vec<char>) -> Vec<Vec<char>> {
pub fn tokenize(input: &str) -> Result<Vec<Token>, IvyError> {
    let weakly_paired = HashMap::from([
        ('(', ')'),
        ('[', ']'),
        ('{', '}'),
    ]);
    let strongly_paired = HashSet::from([
        "'''", //multiline comment
        "\"\"\"", //multiline string
        "'", //char
        "\"", //string
    ]);

    let mut tokens = Vec::new();

    // split the string into a list of list of chars (rather than strings that use u8s)
    let splitted = input
        .split("\n")
        .map(|x| x.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let mut spaces = 0;
    let mut tabs = 0;
    for line in splitted {
        for c in line {
            match c {
                ' ' => spaces += 1,
                '\t' => tabs += 1,
                _ => break,
            }
        }

        // once we're here, we've exhausted the initial spaces and tabs

        // handle initial whitespace tokens
        if spaces > 0 {
            if tabs > 0 {
                // both are greater than 0
                // this should be invalid, only allow spaces or tabs
                return Err(IvyError::MixedWhitespace);
            } else {
                // only spaces
                tokens.push(Token::Spaces(spaces));
            }
        } else {
            if tabs > 0 {
                // only tabs
                tokens.push(Token::Tabs(spaces));
            } else {
                // none are greater than 0
                tokens.push(Token::GenericWhite);
            }
        }

        // handled initial whitespace
    }


    Ok(tokens)
}










