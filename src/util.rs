pub fn quit_error(message: &str) -> ! {
    eprintln!("ERROR: {}", message);
    std::process::exit(1);
}

pub fn ensure(condition: bool, message: &str) {
    if !condition {
        quit_error(message);
    }
}
