use std::process::exit;

use clap::ArgMatches;
use diff_img::BlendMode;
use image::DynamicImage;

#[derive(Debug)]
pub struct Config<'a> {
    pub image1: DynamicImage,
    pub image2: DynamicImage,
    pub filename: Option<&'a String>,
    pub blend_mode: BlendMode,
}

impl<'a> Config<'a> {
    pub fn from_clap_matches(matches: &'a ArgMatches) -> Config<'a> {
        // unwrap() should be safe here because clap does argument validation

        let image1_path = matches.get_one::<String>("image1").unwrap();
        let image2_path = matches.get_one::<String>("image2").unwrap();
        let filename: Option<&String> = matches.get_one::<String>("filename");

        let blend_mode: BlendMode = match matches.get_one::<String>("bias") {
            Some(bias) => match bias.as_str() {
                "bias" => BlendMode::BIAS,
                "hue" => BlendMode::HUE,
                _ => BlendMode::None,
            },
            None => BlendMode::BIAS,
        };

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

        Config {
            image1,
            image2,
            filename,
            blend_mode,
        }
    }
}

fn safe_load_image(filename: &str) -> Result<DynamicImage, String> {
    match image::open(filename) {
        Ok(img) => Ok(img),
        Err(msg) => Err(format!("Error loading image {}: {}", filename, msg)),
    }
}
