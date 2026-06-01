use std::env;
use std::process::{Command, Stdio};
use std::fs::{self, File};
use std::io::Write;
use std::time::{SystemTime, UNIX_EPOCH};


///
///
/// # Panics
///
/// Will panic if it fails to create/write files/directories in user's local `TMP` folder.
///
pub fn compile(rcode: &str, target_dir: &str) {
    let unix_timestamp_str = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis()
        .to_string();

    let main_dir = env::temp_dir().join(format!("holylang-{unix_timestamp_str}"));
    let src_dir = main_dir.join("src");
    fs::create_dir_all(&main_dir)
        .unwrap_or_else(|e| panic!("Compile error: Couldnt create directory `{}`, please check permissions. Error: {:?}", main_dir.display(), e));

    fs::create_dir_all(&src_dir)
        .unwrap_or_else(|e| panic!("Compile error: Couldnt create directory `{}`, please check permissions. Error: {:?}", src_dir.display(), e));

    let cargo_file_path = main_dir.join("Cargo.toml");
    let mut cargo_file = File::create(&cargo_file_path)
        .unwrap_or_else(|e| panic!("Compile error: Couldnt create file `{}`, please check permissions. Error: {:?}", cargo_file_path.display(), e));

    let cargo_content = r#"[package]
name = "holyprogram"
version = "0.0.1"
edition = "2024"

[profile.dev]
opt-level = 0
panic = "abort"

[profile.release]
opt-level = 0
panic = "abort"
"#;

    cargo_file.write_all(cargo_content.as_bytes())
        .unwrap_or_else(|e| panic!("Compile error: Couldnt write to file `{}`, please check permissions. Error: {:?}", cargo_file_path.display(), e));
    
    let main_file_path = src_dir.join("main.rs");
    let mut main_file = File::create(&main_file_path)
            .unwrap_or_else(|e| panic!("Compile error: Couldnt create file `{}`, please check permissions. Error: {:?}", main_file_path.display(), e));

    main_file.write_all(rcode.as_bytes())
        .unwrap_or_else(|e| panic!("Compile error: Couldnt write file `{}`, please check permissions. Error: {:?}", main_file_path.display(), e));

    let compile_proc_output = Command::new("cargo")
        .arg("build")
        .stdout(Stdio::null())
        .stderr(Stdio::piped())
        .current_dir(&main_dir)
        .output()
        .unwrap_or_else(|e| panic!("Compile error: Failed to compile transpiled code! Ensure Rust is correctly installed and try again. Error: {e:?}"));

    let stderr = String::from_utf8_lossy(&compile_proc_output.stderr);

    let mut binary_path = main_dir.clone();
    binary_path.push("target");
    binary_path.push("debug");
    binary_path.push("holyprogram");

    assert!(compile_proc_output.status.success(), 
        "This is likely a compiler bug in the transpiler, which is expected because the transpiler is still very experimental.\nBut please open an issue on Github with the following:\nmain_file: {main_file:#?}\nrcode: {rcode:#}\nstderr: {stderr:#}"
    );


    fs::rename(binary_path, target_dir)
        .unwrap_or_else(|e| panic!("Compile clean-up error: Couldnt move binary from `{}` to `{}`. Error: {:?}", main_dir.display(), target_dir, e));

    fs::remove_dir_all(&main_dir)
        .unwrap_or_else(|e| panic!("Compile clean-up error: Couldnt delete directory `{}`, please check your permissions. Error: {:?}", main_dir.display(), e));

}
