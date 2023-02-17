fn main() {
    let mut array = [1, 2, 3, 4, 5];

    println!("Original array: {:?}", array);
    array[1] = 0;
    array[2] = 100;
    array[4] = -50;
    println!("Changed array: {:?}", array);
}
