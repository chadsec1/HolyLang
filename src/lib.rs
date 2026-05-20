pub mod parser;
pub mod semantic;
pub mod transpiler;
pub mod compile;
pub mod error;
pub mod consts;
pub mod ast;
#[cfg(test)]
mod tests_consts;

pub enum CompileInfo {
    CompileTo(String),
    DoNotCompile
}

pub fn compile(source: &str, compile: CompileInfo) -> String {

    // Parse source code
    let mut ast = parser::parse(&source).expect("Parsing failed");

    println!("Pure AST: {:#?}\n\n\n", ast);

    // Run semantic checks, enforce language rules, modify AST to remove inferred and replace with explicit types, etc.
    semantic::check_semantics(&mut ast).expect("Semantic errors");
    println!("Checked AST: {:#?}\n\n\n", ast);


    // Transpile to Rust code
    let rust_code = transpiler::transpile(&ast);
    println!("Transpiled Rust code: {:#?}\n", rust_code);

    match compile {
        CompileInfo::CompileTo(target_path) => {
            // Compile the transpiled Rust code into target_path.
            compile::compile(&rust_code, &target_path).expect("Compile errors");
        },

        CompileInfo::DoNotCompile => {}
    }

    return rust_code
}

