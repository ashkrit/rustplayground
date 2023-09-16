use std::io::{BufRead, StdinLock};
use clap::{App, Arg};
use regex::Regex;

pub fn main() {
    let args = App::new("grep-lite")
        .version("1.0")
        .about("Searches for patterns")
        .arg(Arg::with_name("pattern").required(true))
        .arg(Arg::with_name("input").required(false))
        .get_matches();

    let pattern = args.value_of("pattern").unwrap();
    let re = regex::Regex::new(pattern).unwrap();

    let input = args.value_of("input").unwrap_or("-");

    println!("Pattern: {} , file {}", pattern,input);

    if input == "-" {
        let stdin = std::io::stdin();
        let handle = stdin.lock();
        process_lines(handle, re);
    }else {
        let f = std::fs::File::open(input).unwrap();
        let reader = std::io::BufReader::new(f);
        process_lines(reader, re);
    }
}

fn process_lines<T:BufRead>(reader: T, re: Regex)  {
    for line_ in reader.lines() {
        let line = line_.unwrap();
        match re.find(&line) {
            Some(_) => println!("{}", line),
            None => (),
        }
    }
}
