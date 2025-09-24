//! cargo-deps: clap="4.6"

use clap::Parser;
use std::fs;
use std::path::Path;

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

    // Replace placeholder with actual content
    let js_content = template_content.replace("{{DATA_PLACEHOLDER}}", &args.content);

    // Ensure output directory exists
    if let Some(parent) = Path::new(&args.output).parent() {
        if !parent.exists() {
            if let Err(e) = fs::create_dir_all(parent) {
                eprintln!("Failed to create output directory: {}", e);
                return;
            }
        }
    }

    // Write to output file
    match fs::write(&args.output, js_content) {
        Ok(_) => println!("JS file created: {}", &args.output),
        Err(e) => eprintln!("Failed to write JS file: {}", e),
    }
}
