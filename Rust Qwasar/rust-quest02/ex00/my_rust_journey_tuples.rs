fn main() {
    let tup = ("This is the way", 1337, true, "biip");
    println!("1 - {}", tup.1);
    println!("0 - {}", tup.0);
    println!("3 - {}", tup.3);
    println!("All {:?}", tup);
}