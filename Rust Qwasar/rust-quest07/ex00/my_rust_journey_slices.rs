fn use_slice(slice: &[i32]) {
    println!("length of slice is {:?}", slice.len());
    println!("{:?}", slice);
}

fn main() {
    let data = [10, 20, 30, 40, 50];
    use_slice(&data[2..5]);
}
