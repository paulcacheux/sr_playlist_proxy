fn main() {
    if let Some(url) = std::env::args().nth(1) {
        println!("URL: {}", url);
    } else {
        eprintln!("Please provide an URL for the base of the playlist");
    }
}
