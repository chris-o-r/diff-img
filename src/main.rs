use clap::{Arg, Command};
use config::{DiffMode, BLEND_MODES, DIFF_MODES};
use diff_img::{calculate_diff_ratio, highlight_changes_with_color, lcs_diff};

use crate::config::PERCEPTUAL_MODES;

pub mod config;
pub mod utils;

static RATE: f32 = 100.0 / 256.0;

fn main() {
    let matches = Command::new("diffimg")
        .version("1.0")
        .about("Diff images")
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
            Arg::new("mode")
                .short('m')
                .long("mode")
                .value_parser(DIFF_MODES)
                .help("diff mode")
                .required(false),
        )
        .arg(
            Arg::new("color")
                .long("color")
                .short('c')
                .default_value("[0,255,0,0]"),
        )
        .arg(
            Arg::new("blend")
                .long("blend")
                .short('b')
                .default_value(BLEND_MODES[1])
                .value_parser(BLEND_MODES),
        )
        .arg(
            Arg::new("perceptual_mode")
                .long("perceptual_mode")
                .short('p')
                .value_parser(config::PERCEPTUAL_MODES)
                .default_missing_value(PERCEPTUAL_MODES[0])
                .help("Perceptual diff mode, only used if --mode is set to 'perceptual'")
                .required(false),
        )
        .arg(
            Arg::new("filename")
                .short('f')
                .long("filename")
                .help("If present, save a diff image to this filename. Required if --mode is set.")
                .default_value(".result.png"),
        )
        .get_matches();

    let config = config::Config::from_clap_matches(&matches);

    let mode = config.mode;
    let file_name: Option<&str> = config.filename.map(|s| s.as_str());

    if mode.is_none() {
        println!(
            "Diff ratio {}",
            calculate_diff_ratio(&config.image1, &config.image2)
        )
    } else if let Some(mode_value) = mode {
        let file_name_unwrapped = file_name.expect("Please provide a file name for diff modes");

        let _s: Result<String, _> = match mode_value {
            DiffMode::MarkWithColor => {
                match highlight_changes_with_color(&config.image1, &config.image2, config.color) {
                    Ok(img) => utils::safe_save_image(img, file_name_unwrapped),
                    Err(msg) => {
                        eprintln!("Error highlighting changes: {}", msg);
                        std::process::exit(1);
                    }
                }
            }
            DiffMode::LCS => match crate::lcs_diff(&config.image1, &config.image2, RATE) {
                Ok(img) => utils::safe_save_image(img, file_name_unwrapped),
                Err(msg) => {
                    eprintln!("Error in LCS diff: {:?}", msg);
                    std::process::exit(1);
                }
            },
            DiffMode::Blend => {
                let img =
                    match diff_img::blend_images(&config.image1, &config.image2, config.blend_mode)
                    {
                        Ok(img) => img,
                        Err(err) => {
                            eprintln!("Error blending images: {}", err);
                            std::process::exit(1);
                        }
                    };

                utils::safe_save_image(img, file_name_unwrapped)
            }
            DiffMode::Perceptual => {
                let perceptual_mode = config
                    .perceptual_mode
                    .unwrap_or(config::PerceptualMode::Diff);

                match perceptual_mode {
                    config::PerceptualMode::Diff => {
                        let img = match diff_img::perceptual::create_perceptual_diff_image(
                            &config.image1,
                            &config.image2,
                            0.8,
                        ) {
                            Ok(img) => img,
                            Err(err) => {
                                eprintln!("Error creating perceptual diff image: {}", err);
                                std::process::exit(1);
                            }
                        };
                        utils::safe_save_image(img, file_name_unwrapped)
                    }

                    config::PerceptualMode::Heatmap => {
                        let img = match diff_img::perceptual::create_perceptual_heatmap(
                            &config.image1,
                            &config.image2,
                        ) {
                            Ok(img) => img,
                            Err(err) => {
                                eprintln!("Error creating perceptual heatmap: {}", err);
                                std::process::exit(1);
                            }
                        };
                        utils::safe_save_image(img, file_name_unwrapped)
                    }
                }
            }
        };
    }
}
