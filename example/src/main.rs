extern crate demo_hack;
use demo_hack::add_one;

fn main() {
    let two = 2;
    let nine = add_one!(two) + add_one!(2 + 3);
    println!("nine = {}", nine);
}
