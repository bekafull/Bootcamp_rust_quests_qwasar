fn init_contacts() -> HashMap<&'static str, &'static str> {
    let mut contacts = HashMap::new();
    contacts.insert("Daniel", "206 555 0100");
    contacts.insert("Jack", "555 206 0100");
    contacts.insert("Samantha", "311 555 0100");
    contacts.insert("Teal'c", "922 555 0100");
    contacts
}
