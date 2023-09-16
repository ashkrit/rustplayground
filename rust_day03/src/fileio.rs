use std::fs::File;
use std::io::BufRead;

pub(crate) fn main() {
    let path = "/Users/ashkrit/_code/github/rustplayground/README.md";
    let mut f = File::open(path).unwrap();
    let mut reader = std::io::BufReader::new(f);

    //brute force
    let mut contents = String::new();
    loop {
        let len = reader.read_line(&mut contents).unwrap();
        if len == 0 {
            break;
        }
        println!("{} ({} bytes)", contents, len);
        contents.truncate(0)
    }


    f = File::open(path).unwrap();
    reader = std::io::BufReader::new(f);
    for line in reader.lines() {
        let text  = line.unwrap();
        println!("{} ({} bytes)", text, text.len());
    }


}