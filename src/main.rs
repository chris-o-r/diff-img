use clap::{Arg, Command};
use diff_img::{calculate_diff_ratio, get_diff_from_images};

pub mod config;

fn main() {
    let blend_mode_values = ["bias", "hue"];

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
        .arg(
            Arg::new("bias")
                .short('b')
                .long("bias")
                .default_missing_value(blend_mode_values[0])
                .value_names(&blend_mode_values)
                .help("Use bias blending mode"),
        )
        .get_matches();

    let config = config::Config::from_clap_matches(&matches);

    let file_name = config.filename.map(|s| s.as_str());

    if let Some(file_name) = file_name {
        match get_diff_from_images(config.image1, config.image2, &file_name, config.blend_mode) {
            Ok(_) => {
                println!("Diff image saved to {}", file_name)
            }
            Err(msg) => println!("Error: {}", msg),
        }
        return;
    } else {
        let diff_ratio = calculate_diff_ratio(config.image1.clone(), config.image2.clone());

        println!("Diff ratio: {}", diff_ratio);
    }
}
