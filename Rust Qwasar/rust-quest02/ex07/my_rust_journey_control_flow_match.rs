use std::env;

fn main() {
let args: Vec<String> = env::args().collect();
match args.len() {
    0 => println!("no arguments"),
    1 => println!("one argument"),
    2 => match args[1].as_str() {
        "hello" => println!("world"),
        "john" | "jenny" | "michael" => println!("name"),
        "northern" => println!("light"),
        "grogu" => println!("This is the way."),
        _ => println!("unknown"),
    },
    _ => println!("too many arguments"),
}
}