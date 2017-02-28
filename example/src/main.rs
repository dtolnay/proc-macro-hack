#[macro_use]
extern crate demo_hack;

macro_rules! two {
    () => {
        add_one!(1)
    }
}

fn main() {
    println!("{}", add_one!(two!()));
}
