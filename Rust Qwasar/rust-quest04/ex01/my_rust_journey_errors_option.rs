fn list_of_plastic(plastic: &str) -> Option<&str> {
    let list_plastics = ["Polyethylene Terephthalate", "High-Density Polyethylene", "Polyvinyl Chloride", "Low-Density Polyethylene", "Polypropylene", "Polystyrene", "Polycarbonate"];

    for item in list_plastics.iter() {
        if item == &plastic {
            return Some(item);
        }
    }
    None
}
