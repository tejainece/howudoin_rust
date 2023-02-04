#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

fn main() {
    let person = Person{ name: "Teja".to_string(), age: 21 };
    println!("{person:?}")
}