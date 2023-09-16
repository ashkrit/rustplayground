#[derive(Debug)]
struct File {
    name: String,
    date: Vec<u8>,
}

struct Hostname(String);

fn connect(host: Hostname) {
    println!("connected to {}", host.0);
}

fn open(f: &mut File) -> bool {
    true
}

fn close(f: &mut File) -> bool {
    true
}

fn read(f: &File, buffer: &mut Vec<u8>) -> usize {
    let mut tmp = f.date.clone();
    let data_length = tmp.len();
    buffer.append(&mut tmp);

    return data_length;
}

fn main() {
    let mut f = File {
        name: String::from("readme.md"),
        date: vec![100, 200, 191, 232],
    };

    println!("{:?}", f);
    println!("{} has {} bytes", f.name, f.date.len());

    let h = Hostname(String::from("localhost"));
    connect(h);

    let mut buffer = vec![];

    open(&mut f);
    let bytes = read(&f, &mut buffer);
    close(&mut f);
    println!("{} bytes read from {:?}", bytes, buffer);
}
