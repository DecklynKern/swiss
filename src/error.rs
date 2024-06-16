pub fn error(message: impl Into<String>) -> ! {
    println!("Error: {}.", message.into());
    std::process::exit(1)
}