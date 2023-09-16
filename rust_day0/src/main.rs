use std::thread;

#[derive(Debug)]
enum Cereal{
    Barley, Millet, Rice,
    Rye, Spelt, Wheat,
}

fn main() {

    //simple_dereference();
    //threads();
}

fn simple_dereference() {
    let mut grains: Vec<Cereal> = vec![];

    grains.push(Cereal::Barley);
    grains.push(Cereal::Millet);

    println!("{:?}", grains);

    //drop(grains); // this line will cause error because it is deallocating the memory
    println!("{:?}", grains);
}

fn threads() {
    let mut data =100;

    thread::spawn(|| {data =200});
    thread::spawn(|| {data =300});

    println!("{}", data);

}





