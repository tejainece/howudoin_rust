struct Value {
    val :i8,
}

trait Printable {
    fn printable_string(&self) -> String;
}

impl Printable for Value {
    fn printable_string(&self) -> String {
        return self.val.to_string();
    }
}

fn inc(val: &mut Value) {
    val.val = val.val + 1;
}

fn main() {
    let mut val = Value{val: 5};
    inc(&mut val);
    val.val += 1;
    println!("{}", val.printable_string());

    let mut printer = || {
        val.val += 1;
        println!("{}", val.val);
    };

    printer();
}
