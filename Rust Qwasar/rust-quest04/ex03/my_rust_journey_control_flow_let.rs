use std::env;

fn main() {
match env::args().nth(1) {
Some(argument) => {
for c in argument.chars() {
println!("{}", c);
}
},
None => {}
}
}




