

use std::thread;

#[derive(Debug)]
enum Cereal{
    Barley, Millet, Rice,
    Rye, Spelt, Wheat,
}

/**
This program demonstrate how Rust provide safety with concurrent access , dereference and array access
none of this will even compile !. This is beauty of rust. Dead code is also not complied if it has got concurrency issue.
**/

fn main() {

    //simple_dereference();
    //threads();
    //array_access();
}

/*
fn simple_dereference() {
    let mut grains: Vec<Cereal> = vec![];

    grains.push(Cereal::Barley);
    grains.push(Cereal::Millet);

    println!("{:?}", grains);

    drop(grains); // this line will cause error because it is deallocating the memory
    println!("{:?}", grains);
}

fn threads() {
    let mut data =100;

    //this block of code cause issue with race condition
    thread::spawn(|| {data =200});
    thread::spawn(|| {data =300});

    println!("{}", data);

}



fn array_access() {
    let fruits = ["Apple", "Banana", "Orange"];
    let over_flow_item = fruits[4]; // this line will cause error because it is out of bound
}


 */




