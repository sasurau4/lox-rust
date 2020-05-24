use clap::{App, Arg};
pub use lexer::Lexer;
use log::info;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};
pub use token_type::TokenType;

mod error;
mod lexer;
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
    let mut lexer = lexer::Lexer::new(source);
    let tokens = lexer.tokenize_all();
    println!("tokens: {:#?}", tokens)
}
