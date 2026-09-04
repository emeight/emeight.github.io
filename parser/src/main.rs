use parser::to_html;
use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .expect("failed to read stdin");
    print!("{}", to_html(&input));
}