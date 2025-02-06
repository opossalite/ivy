use std::collections::{HashMap, HashSet};


pub enum Token {
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



pub fn tokenize(input: &Vec<char>) -> Vec<Vec<char>> {
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


    let ret = Vec::new();
    ret
}










