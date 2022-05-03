use pest::{iterators::Pair, Parser};
use std::fmt;
use rustyline::{self, error::ReadlineError, Editor};

mod error;
use error::LvalResult;

const EXIT_MESSAGE: &str = "Exiting...";

#[macro_use]
extern crate pest_derive;

#[derive(Parser)]
#[grammar = "lisp.pest"]
pub struct RisprParser;

/// Runs an interactive Lispr session in the terminal
pub fn repl() {
    println!("Lispr v-1.0.1");
    println!("Use exit(), Ctrl-C, or Ctrl-D to exit prompt");

    let mut rl = Editor::<()>::new();
    if rl.load_history("./.lispr-history.txt").is_err() {
        println!("No history found.");
    }

    loop {
        let input = rl.readline("lispr> ");

        match input {
            Ok(line) => {
                rl.add_history_entry(&line);
                let result = parse(&line);
                print_ast_results(result)
            }
            Err(ReadlineError::Interrupted) => {
                println!("{}", EXIT_MESSAGE);

                break;
            }
            Err(ReadlineError::Eof) => {
                println!("{}", EXIT_MESSAGE);

                break;
            }
            Err(err) => {
                panic!("Error: {}", err);
            }
        }
    }

    rl.save_history("./.lispr-history.txt").unwrap();
}

fn print_ast_results(result: LvalResult) {
    match result {
        Ok(tree) => println!("{}", tree),
        Err(err) => eprintln!("Error: {}", err),
    }
}

/// Converts a string to a Abstract Syntax Lval
///
/// # Panics
///
/// if the line could not be parsed
fn parse(line: &str) -> LvalResult {
    let tokens = RisprParser::parse(Rule::rispy, &line)?.next().unwrap();

    let ret_tree = rule_to_ast(tokens);

    Ok(ret_tree)
}

fn evaluate_ast(tree: &Lval) -> i64 {
    if let Lval::Sexp(exp) = tree {
        if let Lval::Num(num) = &*exp[0] {
            return *num;
        }
        // First member of expression has to be operator
        else if let Lval::Sym(op) = &*exp[0] {
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

/// Converts the parsed tree of rules into the Lval format
///
/// This consumes the Pair<Rule> in the process
fn rule_to_ast(rule: Pair<Rule>) -> Lval {
    let new_tree = match rule.as_rule() {
        Rule::rispy | Rule::sexp => {
            let mut children = LvalChildren::new(); 

            for child in rule.into_inner() {
                children.push(Box::new(rule_to_ast(child)));
            }

            Lval::Sexp(children)
        },
        Rule::expression => rule_to_ast(rule.into_inner().next().unwrap()),
        Rule::symbol => Lval::Sym(rule.as_str().to_owned()),
        Rule::number => Lval::Num(rule.as_str().parse::<i64>().unwrap()),
    };

    new_tree
}

type LvalChildren = Vec<Box<Lval>>;

pub enum Lval {
    Num(i64),
    Sym(String),
    Sexp(LvalChildren),
}

impl fmt::Display for Lval {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Lval::Num(num) => write!(f, "{}", num),
            Lval::Sym(op) => write!(f, "{}", op),
            Lval::Sexp(children) => write!(f, "({})", print_ast_children(&children)),
        }
    }
}

fn print_ast_children(children: &LvalChildren) -> String {
    let mut ret = String::new();

    for (index, child) in children.iter().enumerate() {
        ret.push_str(&format!("{}", child));

        if index < children.len() - 1 {
            ret.push(' ');
        }
    }

    ret
}
