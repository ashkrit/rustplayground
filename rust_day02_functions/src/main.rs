use std::ops::Add;
use std::time::Duration;

fn main() {
    println!("Total {}!", sum(10, 20));

    let x = 20;
    let y = 30;
    let t = add_with_lifetime(&x, &y);

    println!("Result {}", t);

    let floats = add(10.1, 20.9);
    let total_duration = add(
        Duration::new(5, 0),
        Duration::new(10, 0));


    println!("{}", floats);
    println!("{:?}", total_duration);

}

fn sum(a: i32, b: i32) -> i32 {
    a + b
}

fn add_with_lifetime<'a, 'b>(i: &'a i32, j: &'b i32) -> i32 {
    *i + *j
}

fn add<T: Add<Output=T>>(i: T, j: T) -> T {
    return i + j;
}
