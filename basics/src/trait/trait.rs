trait DonkeyRust {
    fn say_rust_bad(&self);
}

impl DonkeyRust for i8 {
    fn say_rust_bad(&self) {
        println!("Rust is developed by donkeys. Rust developers are Donkeys.")
    }
}

fn main() {
    let int: i8 = 0;
    int.say_rust_bad();
}