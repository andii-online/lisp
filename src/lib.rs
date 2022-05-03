use pest::{iterators::Pair, Parser};
use std::fmt;
use std::fmt::Debug;
use std::hash::Hash;

#[macro_use]
extern crate pest_derive;

#[derive(Parser)]
#[grammar = "lisp.pest"]
pub struct RisprParser;

pub type Result<T> = std::result::Result<T, RisprError>;
pub type AstResult = Result<i64>;

#[derive(Debug, Clone)]
pub enum RisprError {
    ReadLineError(String),
    ParseError(String),
}

impl fmt::Display for RisprError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            RisprError::ParseError(err) => write!(f, "{}", err),
            RisprError::ReadLineError(err) => write!(f, "{}", err),
        }
    }
}

impl<T> From<pest::error::Error<T>> for RisprError
where
    T: Debug + Ord + Copy + Hash,
{
    fn from(error: pest::error::Error<T>) -> Self {
        RisprError::ParseError(format!("{}", error))
    }
}

/// Converts a string to a Abstract Syntax Ast
///
/// # Panics
///
/// if the line could not be parsed
pub fn parse(line: &str) -> AstResult {
    let tokens = RisprParser::parse(Rule::rispy, &line)?.next().unwrap();

    let ret_tree = rule_to_ast(tokens);
    let ret = evaluate_ast(&ret_tree);

    Ok(ret)
}

fn evaluate_ast(tree: &Ast) -> i64 {
    if let Ast::Expression(exp) = tree {
        if let Ast::Number(num) = &*exp[0] {
            return *num;
        }
        // First member of expression has to be operator
        else if let Ast::Operator(op) = &*exp[0] {
            match &op[..] {
                "+" => {
                    let numbers: Vec<i64> = exp[1..]
                        .iter()
                        .map(|child| evaluate_ast(&**child))
                        .collect();
                    let mut sum = 0;
                    for num in numbers {
                        sum += num;
                    }

                    return sum;
                }
                "-" => {
                    let numbers: Vec<i64> = exp[2..]
                        .iter()
                        .map(|child| evaluate_ast(&**child))
                        .collect();
                    let mut sum = evaluate_ast(&*exp[1]);
                    for num in numbers {
                        sum -= num;
                    }

                    return sum;
                }
                "*" => {
                    let numbers: Vec<i64> = exp[1..]
                        .iter()
                        .map(|child| evaluate_ast(&**child))
                        .collect();
                    let mut sum = 1;
                    for num in numbers {
                        sum *= num;
                    }

                    return sum;
                }
                "/" => {
                    let numbers: Vec<i64> = exp[2..]
                        .iter()
                        .map(|child| evaluate_ast(&**child))
                        .collect();
                    let mut sum = evaluate_ast(&*exp[1]);
                    for num in numbers {
                        sum /= num;
                    }

                    return sum;
                }
                _ => (),
            }
        }

        return 0;
    }

    0
}

fn rule_to_ast(rule: Pair<Rule>) -> Ast {
    let new_tree = match rule.as_rule() {
        Rule::rispy => {
            let mut children = AstChildren::new();
            for child in rule.into_inner() {
                children.push(Box::new(rule_to_ast(child)));
            }

            Ast::Expression(children)
        }
        Rule::expression => {
            let mut children = AstChildren::new();
            for child in rule.into_inner() {
                children.push(Box::new(rule_to_ast(child)));
            }

            Ast::Expression(children)
        }
        Rule::operator => Ast::Operator(rule.as_str().to_owned()),
        Rule::number => Ast::Number(rule.as_str().parse::<i64>().unwrap()),
    };

    new_tree
}

type AstChildren = Vec<Box<Ast>>;

pub enum Ast {
    Number(i64),
    Operator(String),
    Expression(AstChildren),
}

impl fmt::Display for Ast {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Ast::Number(num) => write!(f, "{}", num),
            Ast::Operator(op) => write!(f, "{}", op),
            Ast::Expression(children) => write!(f, "({})", print_ast_children(&children)),
        }
    }
}

fn print_ast_children(children: &AstChildren) -> String {
    let mut ret = String::new();

    for (index, child) in children.iter().enumerate() {
        ret.push_str(&format!("{}", child));

        if index < children.len() - 1 {
            ret.push(' ');
        }
    }

    ret
}
