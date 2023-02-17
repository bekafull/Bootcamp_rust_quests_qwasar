macro_rules! my_println {
    ($($arg:tt)*) => (print!("{}\n", format_args!($($arg)*)));
}