use std::fs;

fn main() {
    let source_path = std::env::args().nth(1)
        .expect("No holy source file path provided");

    let target_path = std::env::args().nth(2)
        .expect("No output file path provided");


    let source_content = fs::read_to_string(&source_path)
        .expect("Failed to read source file file, check your permissions.");

    let compile_info = holylang::CompileInfo::CompileTo(target_path);

    holylang::compile(&source_content, compile_info);
}
