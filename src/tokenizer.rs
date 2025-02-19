use std::collections::{HashMap, HashSet};

use crate::IvyError;



const ISOLATE_SYMBOLS: &[&str] = &[
    "="
];


#[derive(Debug)]
pub enum Token {
    Tabs(usize),
    Spaces(usize),
    GenericWhitespace,

    //Parentheses(Vec<Token>),
    //Brackets(Vec<Token>),
    //Braces(Vec<Token>),
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
    //Comma,
    //Dollar,
    //Underscore,


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
    println!("tokenizing: {:?}", input);

    // split the string into a list of list of chars (rather than strings that use u8s)
    let mut splitted = input
        .split("\n")
        .filter(|x| *x != "")
        .map(|x| x.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    //let mut splitted_test = input
    //    .split("\n")
    //    .collect::<Vec<&str>>();

    //let mut splitted_test1 = input
    //    .split("\n")
    //    .filter(|x| *x != "")
    //    .collect::<Vec<&str>>();
    
    //println!("length of splitted: {}", splitted.len());
    //println!("{:?}", splitted);
    //println!("{:?}", splitted_test);
    //println!("{:?}", splitted_test1);

    let mut spaces = 0;
    let mut tabs = 0;

    for i in 0..splitted.len() {
        let line = &splitted[i];

        let mut white_amount = 0;
        for c in line {
            match c {
                ' ' => spaces += 1,
                '\t' => tabs += 1,
                _ => {
                    white_amount = spaces + tabs;
                    splitted[i] = splitted[i].drain(0..white_amount).collect();
                    break;
                },
            }
        }

        let line = &splitted[i]; //update the line after probable draining

        //let clipped_line = splitted.clone().

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
                tokens.push(Token::GenericWhitespace);
            }
        }

        // handled initial whitespace

    }

    Ok(tokens)
}










