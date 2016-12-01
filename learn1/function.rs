fn add(a: i8, b: i8) -> i8 {
    return a + b;
}

fn print_int(num: u8) {
    println!("{}", num);
}

fn main() {
    let a = 5;
    let b = -6;
    println!("{}", add(a, b));
    print_int(5)
}
