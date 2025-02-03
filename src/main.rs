use std::env;
use std::fs;
use std::path::Path;
use std::process::{Command, exit};

fn main() {
    // Get the base directory from command-line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <base_directory>", args[0]);
        exit(1);
    }

    let base_dir = Path::new(&args[1]);
    let locked_dir = base_dir.join("locked");
    let unlocked_dir = base_dir.join("unlocked");

    // Check if qpdf is installed
    if !is_qpdf_installed() {
        eprintln!("Error: 'qpdf' is not installed. Please install it and try again.");
        exit(1);
    }

    // Check if the "locked" directory exists
    if !locked_dir.exists() || !locked_dir.is_dir() {
        eprintln!("Error: '{}' directory does not exist or is not a directory.", locked_dir.display());
        exit(1);
    }

    // Create "unlocked" directory if it doesn't exist
    if !unlocked_dir.exists() {
        match fs::create_dir(&unlocked_dir) {
            Ok(_) => println!("Created missing '{}' directory.", unlocked_dir.display()),
            Err(e) => {
                eprintln!("Error creating '{}': {}", unlocked_dir.display(), e);
                exit(1);
            }
        }
    }

    // Iterate over all PDF files in "locked" directory
    let entries = match fs::read_dir(&locked_dir) {
        Ok(entries) => entries,
        Err(e) => {
            eprintln!("Error reading '{}': {}", locked_dir.display(), e);
            exit(1);
        }
    };

    for entry in entries {
        if let Ok(entry) = entry {
            let path = entry.path();
            if path.extension().and_then(|ext| ext.to_str()) == Some("pdf") {
                let output_path = unlocked_dir.join(path.file_name().unwrap());

                println!("Unlocking: {}", path.display());

                let status = Command::new("qpdf")
                    .arg("--decrypt")
                    .arg(path.as_os_str())
                    .arg(output_path.as_os_str())
                    .status();

                match status {
                    Ok(status) if status.success() => println!("Unlocked: {}", output_path.display()),
                    Ok(_) => eprintln!("Failed to unlock: {}", path.display()),
                    Err(e) => eprintln!("Error running qpdf on '{}': {}", path.display(), e),
                }
            }
        }
    }
}

// Function to check if qpdf is installed
fn is_qpdf_installed() -> bool {
    Command::new("qpdf")
        .arg("--version")
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}
