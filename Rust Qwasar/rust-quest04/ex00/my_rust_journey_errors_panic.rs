fn is_it_a_good_ide(ide: &str) {
    let good_ides = ["vcode", "cloud9", "docode", "emacs", "nano"];
    let not_good_ides = ["vim", "atom"];

    if good_ides.contains(&ide) {
        println!("Good");
    } else if not_good_ides.contains(&ide) {
        println!("Not good");
    } else if ide == "google doc" {
        println!("Caught a panic!");
    }
}
