fn main() {
let a = 10;
let b = 9;
let c = 11;
let d = 10;
let y = 9;
let z = 11;

if a > b && a < c && a == d {
println!("a is bigger than b AND smaller than c AND equal to d");
}
if z > a || y > a {
println!("z OR y are bigger than a");
}
}