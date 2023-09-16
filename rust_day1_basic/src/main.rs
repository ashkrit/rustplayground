use std::iter::zip;
use std::time::{Instant, Duration};


fn main() {
    let tuple = (10, 20, 30);

    println!("{:?}", tuple);
    println!("{:?} {:?}", tuple.0, tuple.2);

    for c in 1..10 {
        println!("{}", c);
    }

    let values = [10, 20, 30];

    for x in values {
        println!("Collection values {}", x);
    }

    for x in 0..values.len() {
        println!("Collection values {}", values[x]);
    }


    let mut samples = vec![];
    while samples.len() < 10 {
        let v = rand::random::<u8>();
        if v > 100 {
            samples.push(v);
        }
    }

    println!("{:?}", samples);

    let start_time = Instant::now();
    let duration = Duration::from_secs(2);

    while Instant::now() - start_time < duration {
        println!("Still some time left {:?}", Instant::now())
    }

    //zip values
    for (x, y) in (0..).zip(0..) {
        println!("Merged Value {}", x + y);
        if x + y == 50 {
            break;
        }
    }


    let hay_stack = [1, 2, 4, 8, 15, 43, 97];
    for item in hay_stack {
        let r = match item {
            15 | 43 => "Meaning of life",
            _ => "Missed",
        };

        if r == "Meaning of life" {
            println!("{}", r);
            break;
        }
    }


    for item in &hay_stack {
        if *item == 43 {
            println!("Found it ");
        }
        println!("{}", item)
    }



}

