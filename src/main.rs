use std::fs;

fn main() {
    // Read file path from CLI
    let path = std::env::args().nth(1).expect("No file provided");
    let source = fs::read_to_string(&path)
        .expect("Failed to read file");

    holylang::compile(&source);
}
