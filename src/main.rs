use colored::*;
use heck::ToKebabCase;
use std::env;
use std::fs;
use std::io::{self};
use std::path::Path;

fn main() -> io::Result<()> {
    let alert_prefix: ColoredString = "Error:".red().bold();

    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: rename-book <file-path>");
        return Ok(());
    }

    let input_path = &args[1];
    let file_path = Path::new(input_path);

    if !file_path.is_file() {
        eprintln!(
            "{} File '{}' is not a valid file path! Cancelling...",
            alert_prefix,
            input_path.yellow()
        );
        return Ok(());
    }

    let original_file_no_ext = match file_path.file_stem() {
        Some(name) => name,
        None => {
            eprintln!("{} Could not extract filename from path!", alert_prefix);
            return Ok(());
        }
    };
    let original_file_stem = original_file_no_ext.to_string_lossy().to_string();

    let file_extension = file_path
        .extension()
        .and_then(|ext| ext.to_str())
        .map(|s| s.to_lowercase());

    match file_extension {
        Some(extension) if extension == "pdf" || extension == "epub" => {
            let renamed_file_stem = original_file_stem.to_kebab_case();
            let renamed_file_name = format!("{}.{}", renamed_file_stem, extension);

            if original_file_stem == renamed_file_stem {
                println!(
                    "Filename is already in kebab-case: '{}'",
                    original_file_stem
                );
                return Ok(());
            }

            let parent_directory = file_path.parent().unwrap_or_else(|| Path::new("./"));
            let new_file_path = parent_directory.join(&renamed_file_name);

            println!(
                "Renaming '{}' to '{}'",
                file_path.display(),
                new_file_path.display()
            );

            match fs::rename(file_path, &new_file_path) {
                Ok(_) => println!(
                    "File renamed successfully to '{}'",
                    new_file_path.display().to_string().green().bold()
                ),
                Err(error) => {
                    eprintln!("{} renaming file: {}", alert_prefix, error);
                    return Err(error);
                }
            }
        }
        Some(_) => {
            println!(
                "Ignoring non-book '{}' (only pdfs and epubs are processed).",
                original_file_stem.cyan()
            );
        }
        None => {
            println!(
                "Ignoring non-book '{}' because it has no extension.",
                original_file_stem.cyan()
            );
        }
    }

    Ok(())
}
