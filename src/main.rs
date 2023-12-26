use crate::gvs::errors::ExtendedUnsvgError;
use clap::Parser;
use std::collections::HashMap;
use std::error::Error;
use unsvg::Image;
pub mod gvs;
use std::env;

/// A simple program to parse four arguments using clap.
#[derive(Parser)]
struct Args {
    file_path: std::path::PathBuf,
    image_path: std::path::PathBuf,
    height: u32,
    width: u32,
}

fn main() -> Result<(), Box<dyn Error>> {
    env::set_var("RUST_BACKTRACE", "1");

    let args: Args = Args::parse();

    // Access the parsed arguments
    let file_path = args.file_path;
    let image_path = args.image_path;
    let height = args.height;
    let width = args.width;

    //setup args
    let mut image = Image::new(width, height);
    let mut turtle = gvs::turtle::Turtle::new(image);
    let mut vec: Vec<Vec<_>> = gvs::parse_path(&file_path);
    let mut dict: HashMap<String, String> = HashMap::new();

    //parse and check for errors, returning variable dict
    gvs::parse_error_check(&mut vec, &mut dict)?;
    //translate and draw
    gvs::translate(&vec, &mut turtle, &mut dict)?;

    //move back into image
    image = turtle.image;
    match image_path.extension().and_then(|s| s.to_str()) {
        Some("svg") => {
            let res = image.save_svg(&image_path);
            if let Err(e) = res {
                return Err(Box::new(e));
            }
        }
        Some("png") => {
            let res = image.save_png(&image_path);
            if let Err(e) = res {
                return Err(Box::new(e));
            }
        }
        _ => {
            eprintln!("File extension not supported");
            return Err(Box::new(ExtendedUnsvgError {
                msg: "File extension not supported".to_string(),
            }));
        }
    }

    Ok(())
}
