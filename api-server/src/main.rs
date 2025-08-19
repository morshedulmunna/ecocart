use ecocart;

fn main() {
    if dotenv::dotenv().is_err() {
        let _ = dotenv::from_filename(".env");
    }
    if let Err(e) = ecocart::execute() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
