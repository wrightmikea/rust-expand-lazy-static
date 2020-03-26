// cargo run # to see DEBUG=false
// cargo run -- -d # to see DEBUG=false
// cargo expand # to see lazy_static expansion
// (first time use: cargo install cargo-expand # to add the expand subcommand)
#[macro_use]
extern crate lazy_static;
use std::env;
lazy_static! {
    static ref DEBUG: bool = env::args().any(|s| s.starts_with("-d"));
}
fn main() {
    println!("hello rust-expand-lazy-static!");
    dbg!(*DEBUG);
}

