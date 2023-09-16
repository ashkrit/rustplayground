
use clap::{App, Arg};
use regex::Regex;

pub fn main() {
    let args = App::new("grep-lite")
        .version("1.0")
        .about("searches for patterns")
        .arg(Arg::with_name("pattern").help("Pattern to search").takes_value(true).required(true))
        .get_matches();

    let pattern_to_search = args.value_of("pattern").unwrap();


    let pattern = Regex::new(pattern_to_search).unwrap();
    let quote = "Every face, every shop, bedroom window, public-house, and
dark square is a picture feverishly turned--in search of what?
It is the same with books. What do we seek through millions of pages?";

    for line in quote.lines() {
        let contains_substring = pattern.find(line);
        match contains_substring {
            Some(_) => println!("{}", line),
            None => (),
        }
    }


}