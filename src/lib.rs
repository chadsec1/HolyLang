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

///
/// # Panics
/// Panics if there are parsig errors, semantic errors, or compile errors.
///
#[allow(clippy::must_use_candidate)]
pub fn compile_holylang_src(source: &str, compile_info: CompileInfo) -> String {
    // Parse source code
    let mut ast = parser::parse(source).expect("Parsing failed");

    println!("Pure AST: {ast:#?}\n\n\n");

    if !ast.functions.iter().any(|f| f.name == "main") {
        panic!("Missing `main` function");
    }

    // Run semantic checks, enforce language rules, modify AST to remove inferred and replace with explicit types, etc.
    semantic::check_semantics(&mut ast).expect("Semantic errors");
    println!("Checked AST: {ast:#?}\n\n\n");


    // Transpile to Rust code
    let rust_code = transpiler::transpile(&ast);
    println!("Transpiled Rust code:\n{rust_code:#}\n");

    match compile_info {
        CompileInfo::CompileTo(target_path) => {
            // Compile the transpiled Rust code into target_path.
            compile::compile(&rust_code, &target_path);
        },

        CompileInfo::DoNotCompile => {}
    }

    rust_code
}

