use std::vec::Vec;

fn status_and_push(vector: &mut Vec<i32>) {
println!("Length of vector: {}", vector.len());
if let Some(first) = vector.first() {
vector.push(*first);
}
}

fn main() {
let mut vector = Vec::<i32>::new();
vector.push(0);
vector.push(1);
status_and_push(&mut vector);
println!("{:?}", vector);
vector.rotate_left(1);
status_and_push(&mut vector);
println!("{:?}", vector);
}