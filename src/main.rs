use clap::{Arg, Command};
use config::{DiffMode, BLEND_MODES, DIFF_MODES};
use diff_img::{highlight_changes_with_color, lcs_diff};

pub mod config;
pub mod lib;

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
            Arg::new("filename")
                .short('f')
                .long("filename")
                .help("If present, save a diff image to this filename."),
        )
        .arg(
            Arg::new("mode")
                .short('m')
                .long("mode")
                .default_value(DIFF_MODES[1])
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
        .get_matches();

    let mut config = config::Config::from_clap_matches(&matches);

    let mode = config.mode;
    let file_name: Option<&str> = config.filename.map(|s| s.as_str());

    let _s: Result<String, _> = match mode {
        DiffMode::MarkWithColor => {
            match highlight_changes_with_color(config.image1, config.image2, config.color) {
                Ok(img) => lib::safe_save_image(img, file_name.unwrap()),
                Err(msg) => {
                    panic!("{}", msg);
                }
            }
        }
        DiffMode::LCS => {
            match crate::lcs_diff(&mut config.image1, &mut config.image2, RATE) {
                Ok(img) => lib::safe_save_image(img, file_name.unwrap()),
                Err(msg) => {
                    panic!("{}", msg);
                }
            }
        }
        DiffMode::Blend => {
            let img =
                diff_img::blend_images(config.image1, config.image2, config.blend_mode).unwrap();

            lib::safe_save_image(img, file_name.unwrap())
        }
    };
}
