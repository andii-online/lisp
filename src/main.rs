use lisp::{RisprError, AstResult, parse};
use rustyline::{self, error::ReadlineError, Editor};

const EXIT_MESSAGE: &str = "Exiting...";

fn main() {
    println!("Rispr v0.0.1");
    println!("Use exit(), Ctrl-C, or Ctrl-D to exit prompt");

    let mut rl = Editor::<()>::new();
    if rl.load_history("./.rispr-history.txt").is_err() {
        println!("No history found.");
    }

    loop {
        let input = rl.readline("rispr> ");

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
}

fn print_ast_results(result: AstResult) {
    match result {
        Ok(tree) => println!("{}", tree),
        Err(err) => eprintln!("Error: {}", err)
    }
}
