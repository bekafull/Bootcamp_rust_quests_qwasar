fn my_initialize(nbr: &mut i16) {
*nbr = 0;
}

fn main() {
let mut nbr = 40;
println!("{}", nbr);
my_initialize(&mut nbr);
println!("{}", nbr);
}