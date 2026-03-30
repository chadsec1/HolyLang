
pub mod parser;
pub mod semantic;
pub mod error;
pub mod consts;
#[cfg(test)]
mod tests_consts;

pub fn compile(source: &str) {

    // Parse source code
    let mut ast = parser::parse(&source).expect("Parsing failed");

    println!("Pure AST: {:#?}\n\n\n", ast);

    // Run semantic checks, enforce language rules, modify AST to remove inferred and replace with explicit types, etc.
    semantic::check_semantics(&mut ast).expect("Semantic errors");
    println!("Checked AST: {:#?}\n\n\n", ast);


    // TODO: Transpile to Rust code
    // let rust_code = transpile(&ast);
}

