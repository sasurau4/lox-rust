use ast_printer::AstPrinter;
use clap::{App, Arg};
use expr::Expr;
use log::info;
use parser::Parser;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};
use token::Token;

mod ast_printer;
mod error;
mod expr;
mod lexer;
mod parser;
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

    // let minus_token = Token {
    //     token_type: TokenType::Minus,
    //     lexeme: "-",
    //     literal: token::Literal::None,
    //     line: 1,
    // };
    // let one_two_three = Expr::Literal {
    //     value: token::Literal::Usize(123),
    // };
    // let star_token = Token {
    //     token_type: TokenType::Star,
    //     lexeme: "*",
    //     literal: token::Literal::None,
    //     line: 1,
    // };
    // let four_five_point = Expr::Grouping {
    //     expression: Box::new(Expr::Literal {
    //         value: token::Literal::Float(45.67),
    //     }),
    // };

    // let unary = Expr::Unary {
    //     operator: minus_token,
    //     right: Box::new(one_two_three),
    // };

    // let expression = Expr::Binary {
    //     left: Box::new(unary),
    //     operator: star_token,
    //     right: Box::new(four_five_point),
    // };

    if let Some(ref in_file) = matches.value_of("input") {
        println!("work for {}", in_file);
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
    for line in f.lines() {
        print!(">");
        source.push_str(&line.unwrap());
        source.push_str("\n")
    }
    run(&source);

    Ok(())
}

fn run_prompt() -> io::Result<()> {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    print!("> ");
    stdout.flush().unwrap();
    let mut source = String::from("");
    for line in stdin.lock().lines() {
        source.push_str(&line.unwrap());
        run(&source);
        source = String::from("");
        print!("> ");
        stdout.flush().unwrap();
    }
    Ok(())
}

fn run(source: &str) {
    let mut lexer = lexer::Lexer::new(String::from(source));
    let tokens = lexer.tokenize_all();
    let mut parser = Parser::new(tokens);
    let expression = match parser.parse() {
        Ok(result) => result,
        _ => return,
    };
    let ast_printer = AstPrinter {};
    println!("result: {}", ast_printer.print(expression));
}
