#[macro_use] extern crate demo_hack;
#[macro_use] extern crate demo_hack_impl;

two_fn!(two);

fn main() {
    let x = two();
    let nine = add_one!(x) + add_one!(2 + 3);
    println!("nine = {}", nine);
}
