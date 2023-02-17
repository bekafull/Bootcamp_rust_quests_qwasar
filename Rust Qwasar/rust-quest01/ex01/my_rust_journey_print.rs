fn main() {
    let age  = 40;
    let name = "Jeannie";
    let tuple = (1, 2);
    let number_with_leading_zero = 9;
    
    println!("Age ({})", age);
    println!("Name ({})", name);
    println!("Tuple ({:?})", tuple);
    println!("Number with leading Zero ({:03})", number_with_leading_zero);
}
