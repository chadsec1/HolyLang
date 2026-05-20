pub mod parser;
pub mod semantic;
pub mod transpiler;
pub mod compile;
pub mod error;
pub mod consts;
pub mod ast;
#[cfg(test)]
mod tests_consts;

pub fn compile(source: &str) {

    // Parse source code
    let mut ast = parser::parse(&source).expect("Parsing failed");

    println!("Pure AST: {:#?}\n\n\n", ast);

    // Run semantic checks, enforce language rules, modify AST to remove inferred and replace with explicit types, etc.
    semantic::check_semantics(&mut ast).expect("Semantic errors");
    println!("Checked AST: {:#?}\n\n\n", ast);


    // Transpile to Rust code
    let rust_code = transpiler::transpile(&ast);
    println!("Transpiled Rust code: {:#?}\n", rust_code);


    // Compile the transpiled Rust code.
    compile::compile(&rust_code).expect("Compile errors");
}

