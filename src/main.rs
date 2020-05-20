use clap::{App, Arg};
use log::info;

fn main() {
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
        1 => log::LevelFilter::Info,
        _ => log::LevelFilter::Debug,
    };
    env_logger::builder().filter_level(log_level).init();
    info!("log_level: {}", log_level);

    if let Some(ref in_file) = matches.value_of("input") {
        println!("work for {}", in_file);
        run_file(in_file)
    } else {
        run_prompt()
    }
}

fn run_file(path: &str) {
    println!("{}", path)
}

fn run_prompt() {
    println!("hoge")
}
