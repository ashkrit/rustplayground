fn main() {
    let tree = sled::open("/Users/ashkrit/_tmp/welcome-to-sled")
        .expect("open");


    let no_of_items = 100;
    for i in 1..no_of_items {
        let v = format!("VAL{}", i);
        let k = format!("{}{}", "KEY", i);
        let _ = tree.insert(k, v.as_str());
    }
    assert_eq!(tree.get(&"KEY1"), Ok(Some(sled::IVec::from("VAL1"))));


    println!("Capacity :{}", tree.len());

    let range = "KEY1".."KEY9";
    println!("Key Count {}", tree.range(range).count());

    let result = tree.get("KEY1");
    println!("{}", as_string(result.unwrap().unwrap()));

}

fn as_string(x: sled::IVec) -> String {
    String::from_utf8(x.to_vec()).unwrap()
}
