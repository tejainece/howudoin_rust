trait FatFart {
    fn fat_fart(&self);
}

#[derive(Debug)]
struct Parent {
    child: Child
}

#[derive(Debug)]
struct Child {

}

impl FatFart for Parent {
    fn fat_fart(&self) {
        self.child.fat_fart();
        println!("And then write this kind of stupid code to get any kind of stupid code");
        println!("HaS-a reLaTionship BetER than is-A relationship. bla bla")
    }
}

impl FatFart for Child {
    fn fat_fart(&self) {
        println!("Rust tards be like: InHeRitance Is bAd reeeeEEEeeeee")
    }
}

fn main() {
    let rustDev = Parent{child: Child{}};
    rustDev.fat_fart();
}