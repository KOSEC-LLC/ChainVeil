//! cargo-deps: clap="4.6", rand="0.8"

use clap::Parser;
use rand::Rng;
use std::fs;
use std::path::{Path, PathBuf};

/// Simple script to inject a string into a JS template
#[derive(Parser)]
struct Args {
    /// String to inject
    #[arg(short, long)]
    content: String,

    /// Path to JS template file
    #[arg(short, long, default_value = "templates/template.js")]
    template: String,

    /// Output JS file path
    #[arg(short, long, default_value = "output/output.js")]
    output: String,

    /// Extra file to XOR encrypt
    #[arg(short = 'f', long)]
    payload: Option<String>,
}

fn main() {
    let args = Args::parse();

    // Check that template file exists
    if !Path::new(&args.template).exists() {
        eprintln!("Template file not found: {}", &args.template);
        return;
    }

    // Read the template file
    let template_content = match fs::read_to_string(&args.template) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Failed to read template file: {}", e);
            return;
        }
    };

    // Replace main placeholder with actual content
    let mut js_content = template_content.replace("{{DATA_PLACEHOLDER}}", &args.content);

    // If extra file is provided, encrypt it
    if let Some(extra_path) = args.payload {
        if Path::new(&extra_path).exists() {
            match xor_encrypt_file(&extra_path, &args.output) {
                Ok((enc_filename, key_hex)) => {
                    println!("Encrypted file written: {}", enc_filename.display());
                    // Replace another placeholder in JS with key
                    js_content = js_content.replace("{{KEY_PLACEHOLDER}}", &key_hex);
                }
                Err(e) => {
                    eprintln!("Failed to encrypt file: {}", e);
                    return;
                }
            }
        } else {
            eprintln!("Extra file not found: {}", extra_path);
            return;
        }
    }

    // Ensure output directory exists
    if let Some(parent) = Path::new(&args.output).parent() {
        if !parent.exists() {
            if let Err(e) = fs::create_dir_all(parent) {
                eprintln!("Failed to create output directory: {}", e);
                return;
            }
        }
    }

    // Write to output JS file
    match fs::write(&args.output, js_content) {
        Ok(_) => println!("JS file created: {}", &args.output),
        Err(e) => eprintln!("Failed to write JS file: {}", e),
    }
}

/// Encrypts a file with XOR and writes it to same dir as JS output.
/// Returns (encrypted file path, hex-encoded key).
fn xor_encrypt_file(extra_path: &str, js_output: &str) -> std::io::Result<(PathBuf, String)> {
    let data = fs::read(extra_path)?;

    // Generate random key bytes (same length as file)
    let mut rng = rand::thread_rng();
    let key: Vec<u8> = (0..data.len()).map(|_| rng.r#gen::<u8>()).collect();

    // XOR the file contents
    let encrypted: Vec<u8> = data.iter().zip(&key).map(|(b, k)| b ^ k).collect();

    // Derive encrypted filename in same dir as JS output
    let js_dir = Path::new(js_output)
        .parent()
        .unwrap_or_else(|| Path::new("."));
    let extra_filename = Path::new(extra_path)
        .file_name()
        .unwrap_or_default();
    let encrypted_filename = js_dir.join(format!(
        "{}.enc",
        extra_filename.to_string_lossy()
    ));

    // Write encrypted file
    fs::write(&encrypted_filename, &encrypted)?;

    // Return file path and hex key
    Ok((encrypted_filename, hex::encode(key)))
}
