mod parser;
mod semantic;
mod error;
mod consts;

use std::fs;

fn main() {
    // Read file path from CLI
    let path = std::env::args().nth(1).expect("No file provided");
    let source = fs::read_to_string(&path)
        .expect("Failed to read file");

    // Parse source code
    let mut ast = parser::parse(&source).expect("Parsing failed");

    println!("Pure AST: {:#?}\n\n\n", ast);

    // Run semantic checks, modify AST to remove inferred and replace with explicit types.
    semantic::check_semantics(&mut ast).expect("Semantic errors");
    println!("Checked AST: {:#?}\n\n\n", ast);


    // Transpile to Rust code
    // let rust_code = transpile(&ast);

    // For now, just print Rust code
    // println!("{}", rust_code);
}
