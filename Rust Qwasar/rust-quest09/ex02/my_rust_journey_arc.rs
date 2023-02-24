fn print_reference_count<T>(arc: &Arc<T>) {
    let count = Arc::strong_count(arc);
    println!("ARC count => {}", count);
}
