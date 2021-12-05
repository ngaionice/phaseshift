use std::{process, fs, path, io};

use image::DynamicImage;
use image::io::Reader as ImageReader;
use image::error::{ImageResult};
use image::imageops::FilterType;

/// Processes the input. 
pub fn process_input(command: &str) -> Result<(), io::Error>{
    let vars = command.split(" ").collect::<Vec<&str>>();
    if vars.len() >= 2  {
        if !vars[1].eq("-b") {
            println!("Invalid second argument: the only accepted argument is -b.");
        } else if !fs::metadata(vars[0]).unwrap().is_dir() {
            println!("Batch processing can only be done on a directory.");
        } else {
            let mut output_path = fs::canonicalize(vars[0])?;
            output_path.push("out");
            create_output_folder(&output_path);

            for file_path in fs::read_dir(vars[0]).unwrap() {
                let path = file_path.unwrap().path();
                if path.is_file() {
                    process_file(path);
                }
            }
        }
    } else {
        match fs::metadata(vars[0]) {
            Ok(v) => if v.is_file() {
                let mut output_path = fs::canonicalize(vars[0])?;
                output_path.pop();
                output_path = output_path.join("out");
                create_output_folder(&output_path);

                process_file(path::PathBuf::from(vars[0]));
            } else {
                println!("The specified path does not point to a file.");
            }
            Err(_) => println!("The specified path does not point to a file.")
        }
    }
    Ok(())
}

/// Checks if the input path is a directory; if not, creates a directory at the input path.
fn create_output_folder(output_path: &path::PathBuf) {
    if !fs::metadata(output_path).is_ok() {
        match fs::create_dir_all(output_path) {
            Ok(_) => (),
            Err(e) => {
                eprintln!("{}",e);
                println!("Failed to create output folder.");
                process::exit(1);
            }
        }
    }
}

/// Processes the input image and saves it in a subdirectory called 'out' relative to the input file's directory.
/// Assumes the specified path exists and is a file.
fn process_file(path: path::PathBuf) {
    let output_path = get_output_path(&path);
    let output_image = match read_and_resize(&path) {
        Ok(v) => v,
        Err(_) => {
            println!("Failed to resize {}", &path.to_string_lossy());
            return;
        }
    };
    export(output_path, output_image);
}

/// Takes in a file path of an image file, and returns an ImageResult<DynamicImage> with a maximum height and width of 56 pixels while preserving the aspect ratio.
fn read_and_resize(file_path: &path::PathBuf) -> ImageResult<DynamicImage> {
    let input = ImageReader::open(file_path)?.decode()?;
    Ok(input.resize(56, 56, FilterType::CatmullRom))
}

/// Saves the input DynamicImage to the location specified by the input PathBuf.
fn export(file_path: path::PathBuf, image: DynamicImage) {
    match image.save(&file_path) {
        Ok(_) => (),
        Err(_) => {
            println!("Failed to save to {}", &file_path.to_string_lossy());
        }
    }
}

fn get_output_path(input_path: &path::PathBuf) -> path::PathBuf {
    let mut output_path = path::PathBuf::from(&input_path);
    let mut file_name = get_filename_no_ext(&output_path);
    file_name.push_str(".gif");

    output_path.pop();
    output_path.join("out").join(file_name)
}

/// Returns the input PathBuf's file stem as a String.
fn get_filename_no_ext(path: &path::PathBuf) -> String {
    String::from(path.file_stem().unwrap().to_string_lossy())
}

