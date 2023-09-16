

#[derive(Debug)]
enum Cereal{
    Barley, Millet, Rice,
    Rye, Spelt, Wheat,
}

fn main() {

    simple_dereference();

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
    let data =100;
}





