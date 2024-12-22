use clap::{Arg, Command};
use config::DiffMode;
use diff_img::{lcs_diff, mark_diff_with_color};

pub mod config;
pub mod lib;

static RATE: f32 = 100.0 / 256.0;


const BLEND_MODE_VALUES: [&str; 3] = ["solid-color", "lcs", "blend"];

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
        .arg(
            Arg::new("mode")
                .short('m')
                .long("mode")
                .default_value(BLEND_MODE_VALUES[1])
                .value_parser(BLEND_MODE_VALUES)
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
                .default_value("hue"),
        )
        .get_matches();

    let mut config = config::Config::from_clap_matches(&matches);

    let mode = config.mode;
    let file_name: Option<&str> = config.filename.map(|s| s.as_str());

    let _s: Result<String, _> = match mode {
        DiffMode::MarkWithColor => {
            match mark_diff_with_color::mark_diff_with_color(
                config.image1,
                config.image2,
                config.color,
            ) {
                Ok(img) => {
                    lib::safe_save_image(img, file_name.unwrap())
                }
                Err(msg) => {
                    panic!("{}", msg);
                }
            }
        }
        DiffMode::LCS => {
            match crate::lcs_diff::compare(&mut config.image1, &mut config.image2, RATE) {
                Ok(img) => {
                    lib::safe_save_image(img, file_name.unwrap())
                }
                Err(msg) => {
                    panic!("{}", msg);
                }
            }
        }
        DiffMode::Blend => {
            let img = diff_img::blend_diff::get_diff_from_images(
                config.image1,
                config.image2,
                config.blend_mode,
            )
            .unwrap();

            lib::safe_save_image(img, file_name.unwrap())
        }
    };
}
