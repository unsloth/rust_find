fn main() {
    if let Err(e) = rust_find::run() {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
