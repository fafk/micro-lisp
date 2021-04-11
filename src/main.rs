/// Micro lispesque language
///
/// A tiny interpreter for a language that doesn't do much.
///
/// Steps:
/// * Chop up an input file with the lang source code into lexical units (tokens)
/// * Perform "parsing" phase by creating a "tree" by nesting `Vec`s according to the parentheses
/// * Evaluate the node of the tree
///
/// Every function (if, while, do, ...) returns a value.
///
/// Running the program:
/// `cargo run -- ./examples/loop.mlsp`
///
mod arithmetic;

use crate::Token::{Close, False, Int, List, Open, Symbol, True};
use regex::Regex;
use std::collections::HashMap;
use std::{env, fs};

#[derive(Hash, Eq, Debug, Clone)]
enum Token {
    Open,
    Close,
    Int(i32),
    Symbol(String),
    List(Vec<Token>),
    True,
    False,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Invalid number of arguments. Expected 1 argument with source code file.");
        return;
    }
    let contents =
        fs::read_to_string(&args[1]).expect("Something went wrong reading the source file");

    run(contents);
}

fn run(text: String) -> Vec<Token> {
    // Tokenize!
    let lexer = Lexer::new(text);
    // Parse!
    let ast = parse(lexer);
    // Evaluate!
    ast.into_iter().fold(vec![], |mut acc, node| {
        acc.push(evaluate(&node, &mut HashMap::new()));
        acc
    })
}

fn parse(lexer: Lexer) -> Vec<Token> {
    let mut list_stack: Vec<Vec<Token>> = vec![vec![]];
    let mut curr_list = 0;

    for token in lexer {
        match token {
            Open => {
                list_stack.push(vec![]);
                curr_list = curr_list + 1;
            }
            Close => {
                let last = list_stack.pop().unwrap();
                curr_list = curr_list - 1;
                list_stack[curr_list].push(List(last));
            }
            Token::Int(_) | Symbol(_) | List(_) => {
                list_stack[curr_list].push(token);
            }
            _ => panic!("unrecognized token in parsing"),
        }
    }

    if list_stack.len() != 1 {
        panic!("unmatched parenthesis");
    }

    list_stack.into_iter().flatten().collect()
}

fn evaluate(node: &Token, vars: &mut HashMap<Token, Token>) -> Token {
    match node {
        Open => panic!("open symbol in AST makes no sense"),
        Close => panic!("open symbol in AST makes no sense"),
        Int(number) => Int(number.to_owned()),
        Symbol(symbol) => {
            match vars.get(&Symbol(symbol.to_string())) {
                None => Symbol(symbol.to_string()), //panic!("unknown symbol"),
                Some(value) => value.clone(),
            }
        }
        List(list) => match list.first().unwrap() {
            Symbol(symbol) => match symbol.as_str() {
                "+" => evaluate(&list[1], vars) + evaluate(&list[2], vars),
                "-" => evaluate(&list[1], vars) - evaluate(&list[2], vars),
                "*" => evaluate(&list[1], vars) * evaluate(&list[2], vars),
                ">" => remap_bool(evaluate(&list[1], vars) > evaluate(&list[2], vars)),
                "<" => remap_bool(evaluate(&list[1], vars) < evaluate(&list[2], vars)),
                "=" => remap_bool(evaluate(&list[1], vars) == evaluate(&list[2], vars)),
                "if" => {
                    if let True = evaluate(&list[1], vars) {
                        evaluate(&list[2], vars)
                    } else {
                        evaluate(&list[3], vars)
                    }
                }
                "while" => {
                    let mut value = False;
                    while let True = evaluate(&list[1], vars) {
                        value = evaluate(&list[2], vars);
                    }
                    value
                }
                "do" => {
                    List(list[1..].iter().fold(vec![], |mut acc, node| {
                        acc.push(evaluate(&node, vars));
                        acc
                    }))
                }
                "set" => {
                    let value = evaluate(&list[2], vars);
                    vars.insert(list[1].clone(), value.clone());
                    value
                }
                "print" => {
                    let value = evaluate(&list[1], vars);
                    println!("{:?}", value.clone());
                    value
                }
                _ => match vars.get(&Symbol(symbol.to_string())) {
                    None => panic!("unknown symbol"),
                    Some(value) => value.clone(),
                },
            },
            _ => {
                eprintln!("LIST {:?}", list);
                panic!("can't evaluate list, first item needs to be a symbol");
            }
        },
        True => True,
        False => False,
    }
}

fn remap_bool(value: bool) -> Token {
    if value {
        return True;
    }
    return False;
}

struct Lexer {
    text: String,
    line: usize,
    current_pos: usize,
    token_matcher: TokenMatcher,
}

impl Lexer {
    pub fn new(text: String) -> Self {
        Self {
            text,
            line: 0,
            current_pos: 0,
            token_matcher: TokenMatcher::new(),
        }
    }
}

impl Iterator for Lexer {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        let slice = &self.text[self.current_pos..];
        if slice.is_empty() {
            return None;
        }
        if self.token_matcher.open.is_match(slice) {
            self.current_pos = self.current_pos + 1;
            return Some(Token::Open);
        } else if self.token_matcher.close.is_match(slice) {
            self.current_pos = self.current_pos + 1;
            return Some(Token::Close);
        } else if let Some(m) = self.token_matcher.int.find(slice) {
            self.current_pos = self.current_pos + m.end();
            let number_str = &slice[0..m.end()];
            let number = number_str.parse::<i32>().unwrap();
            return Some(Token::Int(number));
        } else if let Some(m) = self.token_matcher.symbol.find(slice) {
            self.current_pos = self.current_pos + m.end();
            return Some(Token::Symbol(slice[0..m.end()].to_string()));
        } else if let Some(m) = self.token_matcher.newline.find(slice) {
            self.current_pos = self.current_pos + m.end();
            self.line = self.line + 1;
            return self.next();
        } else if let Some(m) = self.token_matcher.whitespace.find(slice) {
            self.current_pos = self.current_pos + m.end();
            return self.next();
        }
        panic!(
            "unrecognized symbol at line {} position {}",
            self.line, self.current_pos
        );
    }
}

struct TokenMatcher {
    open: Regex,
    close: Regex,
    int: Regex,
    symbol: Regex,
    newline: Regex,
    whitespace: Regex,
}

impl TokenMatcher {
    pub fn new() -> Self {
        Self {
            open: Regex::new(r"^\(").unwrap(),
            close: Regex::new(r"^\)").unwrap(),
            int: Regex::new(r"^[\+\-]?[0-9]+").unwrap(),
            symbol: Regex::new(r"^[+\-\*><=a-zA-Z][a-zA-Z0-9]*").unwrap(),
            newline: Regex::new(r"^\n").unwrap(),
            whitespace: Regex::new(r"^\s+").unwrap(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn arith() {
        let text = "(+ (- 10 5) (* 2 2))";
        let res = run(text.to_string());
        assert!(matches!(res[0], Token::Int(9)));

        let text = "(+ (- 10 5) (* -2 10))";
        let res = run(text.to_string());
        assert!(matches!(res[0], Token::Int(-15)));
    }

    #[test]
    fn branching() {
        let text = "(if (> 10 (* 3 3)) 1 2)";
        let res = run(text.to_string());
        assert!(matches!(res[0], Token::Int(1)));

        let text = "(if (< 10 (* 3 3)) 1 2)";
        let res = run(text.to_string());
        assert!(matches!(res[0], Token::Int(2)));
    }

    #[test]
    fn iteration() {
        let text = r#"
            (do
                (set i 5)
                (while (> i 0) (do (print i) (set i (- i 1)))))
            "#;
        let res = run(text.to_string());
        assert!(matches!(res[0], Token::List(_)));
    }
}