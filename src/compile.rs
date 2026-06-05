use crate::error::HolyError;
use std::env;
use std::process::{Command, Stdio};
use std::fs::{self, File};
use std::io::Write;
use std::time::{SystemTime, UNIX_EPOCH};


pub fn compile(rcode: &str, target_dir: &str) -> Result<(), HolyError> {
    let unix_timestamp_str = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis()
        .to_string();

    let main_dir = env::temp_dir().join(format!("holylang-{}", unix_timestamp_str));
    let src_dir = main_dir.join("src");
    fs::create_dir_all(&main_dir).expect(&format!("Compile error: Couldnt create directory `{}`, please check permissions.", main_dir.display()));
    fs::create_dir_all(&src_dir).expect(&format!("Compile error: Couldnt create directory `{}`, please check permissions.", src_dir.display()));

    let cargo_file_path = main_dir.join("Cargo.toml");
    let mut cargo_file = File::create(&cargo_file_path).expect(&format!("Compile error: Couldnt create file `{}`, please check your permissions", cargo_file_path.display()));

    let cargo_content = r#"[package]
name = "holyprogram"
version = "0.0.1"
edition = "2024"
"#;

    cargo_file.write_all(cargo_content.as_bytes()).expect(&format!("Compile error: Couldnt write to file `{}`, please check your permissions", cargo_file_path.display()));
    
    let main_file_path = src_dir.join("main.rs");
    let mut main_file = File::create(&main_file_path).expect(&format!("Compile error: Couldnt create file `{}`, please check your permissions", main_file_path.display()));

    main_file.write_all(rcode.as_bytes()).expect(&format!("Compile error: Couldnt write to file `{}`, please check your permissions", main_file_path.display()));

    let compile_proc_output = Command::new("cargo")
        .arg("build")
        .stdout(Stdio::null())
        .stderr(Stdio::piped())
        .current_dir(&main_dir)
        .output().expect("Compile error: Failed to compile transpiled code! Ensure Rust is correctly installed and try again");

    let stderr = String::from_utf8_lossy(&compile_proc_output.stderr);

    let mut binary_path = main_dir.clone();
    binary_path.push("target");
    binary_path.push("debug");
    binary_path.push("holyprogram");

    if !compile_proc_output.status.success() {
        panic!(
            "This is likely a compiler bug in the transpiler, which is expected because the transpiler is still very experimental.\nBut please open an issue on Github with the following:\nmain_file: {:#?}\nrcode: {:#}\nstderr: {:#}",
            main_file, rcode, stderr
        )
    }

    fs::rename(binary_path, target_dir).expect(&format!("Compile clean-up error: Couldnt move binary from `{}` to `{}`", main_dir.display(), target_dir));
    fs::remove_dir_all(&main_dir).expect(&format!("Compile clean-up error: Couldnt delete directory `{}`, please check your permissions", main_dir.display()));
    Ok(())
}
