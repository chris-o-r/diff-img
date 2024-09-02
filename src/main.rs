use std::process::exit;

use clap::{Arg, Command};
use diff_img::{calculate_diff_ratio, get_diff_from_images, BlendMode};
use image::DynamicImage;

fn main() {
    let matches = Command::new("diffimg")
        .version("1.0")
        .about("Does awesome things")
        .arg(
            Arg::new("image1")
                .help("First image to diff")
                .required(true),
        )
        .arg(
            Arg::new("image2")
                .help("Second image to diff")
                .required(true),
        )
        .arg(
            Arg::new("filename")
                .short('f')
                .long("filename")
                .help("If present, save a diff image to this filename."),
        )
        .get_matches();

    let image1_path = matches.get_one::<String>("image1").unwrap();
    let image2_path = matches.get_one::<String>("image2").unwrap();
    let filename = matches.get_one::<String>("filename");

    let image1 = match safe_load_image(image1_path) {
        Ok(img) => img,
        Err(msg) => {
            println!("Error: {}", msg);
            exit(1);
        }
    };

    let image2 = match safe_load_image(image2_path) {
        Ok(img) => img,
        Err(msg) => {
            println!("Error: {}", msg);
            exit(1);
        }
    };

    if let Some(filename) = filename {
        match get_diff_from_images(image1, image2, &filename, BlendMode::BIAS) {
            Ok(_) => println!("Diff image saved to {}", filename),
            Err(msg) => println!("Error: {}", msg),
        }
        return;
    } else {
        let diff_ratio = calculate_diff_ratio(image1, image2);

        println!("Diff ratio: {}", diff_ratio);
    }
}

fn safe_load_image(filename: &str) -> Result<DynamicImage, String> {
    match image::open(filename) {
        Ok(img) => Ok(img),
        Err(msg) => Err(msg.to_string()),
    }
}
