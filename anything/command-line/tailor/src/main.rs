fn main() {
    if let Err(e) = tailor::get_args().and_then(tailor::run) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
