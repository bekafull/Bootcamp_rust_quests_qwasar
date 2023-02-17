fn concat_string(a: &str, b: &str) -> String {
format!("{}{}", a, b)
}

fn main() {
let hello = String::from("Hello");
let world = String::from("world");
let space = String::from(" ");
let exclamation = String::from("!");


let mut temporary = concat_string(&hello, &space);
temporary = concat_string(&temporary, &world);

println!("{}", concat_string(&temporary, &exclamation));
}