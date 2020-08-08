#![allow(unused_macros)]

macro_rules! testing {
    () => {};
}

fn main() {
    testing!();
    println!("Hello, world!");
}
