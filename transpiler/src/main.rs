mod parser;

use std::fs::File;
use std::io::Read;

fn main() {
    let mut f = File::open(std::env::args().nth(1).unwrap()).unwrap();
    let mut code = String::new();

    f.read_to_string(&mut code).unwrap();

    println!("{}", parser::transpile(&code));
}
