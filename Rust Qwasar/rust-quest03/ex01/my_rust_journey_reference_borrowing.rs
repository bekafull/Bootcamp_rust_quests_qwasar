fn my_formatting_and_printing_function(volcano: &str) {
println!("Volcano Name: {}", volcano);
}

fn main() {
let array = ["Kilauea", "Maunaloa", "Maunakea", "Hualalai", "Kohala", "Leahi", "Oahu", "Haleakala"];
for volcano in array.iter() {
    my_formatting_and_printing_function(volcano);
}
println!("{:?}", array);
}