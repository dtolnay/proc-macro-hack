#[macro_use]
extern crate demo_hack;

two_fn!(two);

fn main() {
    let nine = add_one!(two()) + add_one!(2 + 3);
    println!("nine = {}", nine);
}
