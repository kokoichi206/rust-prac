fn main() {
    if let Err(e) = catwalk::get_args().and_then(catwalk::run) { 
        eprintln!("{}", e); 
        std::process::exit(1); 
    }
}
