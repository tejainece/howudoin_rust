trait FatFart {
    fn fat_fart(&self);
}

trait DumbFuck: FatFart {
    fn dumb_fuck(&self);
}

#[derive(Debug)]
struct RustDev {

}

impl FatFart for RustDev {
    fn fat_fart(&self) {
        println!("Rust tards be like: InHeRitance Is bAd reeeeEEEeeeee")
    }
}

impl DumbFuck for RustDev {
    fn dumb_fuck(&self) {
        println!("Rust dumb fucks have to write multiple impl clauses to implement each trait. SMH 🤮")
    }
}

fn main() {
    let rustDev = RustDev{};
    rustDev.fat_fart();
    rustDev.dumb_fuck();
}