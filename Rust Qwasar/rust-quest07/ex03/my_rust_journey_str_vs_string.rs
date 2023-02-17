fn your_beautiful_function(hello1: &str, hello2: String, hello3: String, hello4: String) {
    printer(hello1);
    printer(&hello2);
    printer(&hello3);
    printer(&hello4);
}
