// use ast_printer::AstPrinter;
use clap::{App, Arg};
use environment::Environment;
use error::{Error, Result};
use interpreter::Interpreter;
use log::{debug, error, info};
use parser::Parser;
use resolver::Resolver;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};
use std::process::exit;

// mod ast_printer;
mod callable;
mod environment;
mod error;
mod expr;
mod interpreter;
mod lexer;
mod lox_class;
mod lox_instance;
mod object;
mod parser;
mod resolver;
mod stmt;
mod token;
mod token_type;

fn main() -> io::Result<()> {
    let matches = App::new("lox-rust")
        .version("0.1")
        .author("Daiki Ihara <sasurau4@gmail.com>")
        .about("lox rust")
        .arg(
            Arg::with_name("input")
                .about("the input file to run")
                .index(1)
                .required(false),
        )
        .arg(
            Arg::new("debug")
                .long("debug")
                .short('d')
                .multiple(true)
                .takes_value(false)
                .about("Turn debugging information on"),
        )
        .get_matches();

    let log_level = match matches.occurrences_of("debug") {
        0 => log::LevelFilter::Error,
        1 => log::LevelFilter::Debug,
        _ => log::LevelFilter::Debug,
    };
    env_logger::builder().filter_level(log_level).init();
    info!("log_level: {}", log_level);

    if let Some(ref in_file) = matches.value_of("input") {
        debug!("run for {}", in_file);
        run_file(in_file)?
    } else {
        run_prompt()?
    }

    Ok(())
}

fn run_file(path: &str) -> io::Result<()> {
    let f = File::open(path)?;
    let f = BufReader::new(f);
    let mut source = String::from("");
    let env = Environment::new(None, false);

    let mut interpreter = Interpreter::new(env);

    for line in f.lines() {
        source.push_str(&line.unwrap());
        source.push_str("\n")
    }
    if let Err(e) = run(&source, &mut interpreter) {
        error!("{:?}", e);
        exit(70);
    };

    Ok(())
}

fn run_prompt() -> io::Result<()> {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    print!("> ");
    stdout.flush().unwrap();
    let env = Environment::new(None, true);
    let mut interpreter = Interpreter::new(env);
    let mut source = String::from("");

    for line in stdin.lock().lines() {
        source.push_str(&line.unwrap());
        if let Err(e) = run(&source, &mut interpreter) {
            error!("{:?}", e);
        };
        source = String::from("");
        print!("> ");
        stdout.flush().unwrap();
    }
    Ok(())
}

fn run(source: &str, interpreter: &mut Interpreter) -> Result<()> {
    let mut lexer = lexer::Lexer::new(String::from(source));
    let tokens = lexer.tokenize_all();
    let mut parser = Parser::new(tokens);
    let statements = match parser.parse() {
        Ok(result) => result,
        _ => return Err(Error::ParseError(String::from("parse error"))),
    };

    let mut resolver = Resolver::new(interpreter);
    resolver.resolve_statements(&statements)?;

    // let ast_printer = AstPrinter {};
    // println!("AST result: {}", ast_printer.print(statements.clone()));
    interpreter.clone().interpret(statements)
    // println!("Evaluated result: {:?}", result);
}
