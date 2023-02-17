fn main() {
let gift1 = Box::new(String::from("Skateboard"));
let gift2 = Box::new(String::from("RC car"));
let gift3 = Box::new(String::from("A book"));
print_gift(gift1);
print_gift(gift2);
print_gift(gift3);
}