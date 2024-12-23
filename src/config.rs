use std::process::exit;

use clap::ArgMatches;
use diff_img::BlendMode;
use image::{DynamicImage, Rgba};

pub const DIFF_MODES: [&str; 3] = ["solid-color", "lcs", "blend"];
pub const BLEND_MODES: [&str; 3] = ["bias", "hue", "overlay"];

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum DiffMode {
    Blend,
    MarkWithColor,
    LCS,
}

#[derive(Debug)]
pub struct Config<'a> {
    pub image1: DynamicImage,
    pub image2: DynamicImage,
    pub filename: Option<&'a String>,
    pub mode: Option<DiffMode>,
    pub blend_mode: BlendMode,
    pub color: Rgba<u8>,
}

impl<'a> Config<'a> {
    pub fn from_clap_matches(matches: &'a ArgMatches) -> Config<'a> {
        // unwrap() should be safe here because clap does argument validation

        let image1_path = matches.get_one::<String>("image1").unwrap();
        let image2_path: &String = matches.get_one::<String>("image2").unwrap();
        let filename: Option<&String> = matches.get_one::<String>("filename");
        let mode_string = matches.get_one::<String>("mode");
        let color_string = matches.get_one::<String>("color").unwrap();

        let mode: Option<DiffMode> = match mode_string {
            Some(val) => match get_mode_from_string(val) {
                Ok(mode) => Some(mode),
                Err(err) => panic!("{}", err),
            },
            None => None,
        };

        let blend_mode: BlendMode = match matches.get_one::<String>("blend") {
            Some(bias) => match string_into_blend_mode(bias) {
                Ok(mode) => mode,
                Err(err) => panic!("{}", err),
            },
            None => BlendMode::Overlay,
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

        let color = rgba_from_string(color_string.as_str()).unwrap();

        Config {
            image1,
            image2,
            filename,
            blend_mode,
            mode,
            color,
        }
    }
}

fn safe_load_image(filename: &str) -> Result<DynamicImage, String> {
    match image::open(filename) {
        Ok(img) => Ok(img),
        Err(msg) => Err(format!("Error loading image {}: {}", filename, msg)),
    }
}

fn get_mode_from_string(input: &str) -> Result<DiffMode, String> {
    match input {
        val if val == DIFF_MODES[0] => Ok(DiffMode::MarkWithColor),
        val if val == DIFF_MODES[1] => Ok(DiffMode::LCS),
        val if val == DIFF_MODES[2] => Ok(DiffMode::Blend),
        _ => Err(format!("Nothing matching {}", input)),
    }
}

fn rgba_from_string(input: &str) -> Result<Rgba<u8>, String> {
    let mut cleaned = input.to_string();
    cleaned = cleaned.replace("[", "");
    cleaned = cleaned.replace("]", "");

    let parts: Vec<u8> = cleaned
        .split(',')
        .map(str::trim) // Trim any extra spaces
        .map(|s| s.parse::<u8>()) // Parse each part as u8
        .collect::<Result<_, _>>()
        .map_err(|err| err.to_string())?; // Collect results, propagate error if any

    // Convert the parsed parts into an array with four elements
    let arr = [
        parts.get(0).copied().unwrap_or(0), // First element, or default to 0
        parts.get(1).copied().unwrap_or(0), // Second element, or default to 0
        parts.get(2).copied().unwrap_or(0), // Third element, or default to 0
        parts.get(3).copied().unwrap_or(0),
    ];

    Ok(Rgba::<u8>(arr))
}

fn string_into_blend_mode(input: &str) -> Result<BlendMode, String> {
    match input {
        val if val == BLEND_MODES[0] => Ok(BlendMode::BIAS),
        val if val == BLEND_MODES[1] => Ok(BlendMode::HUE),
        val if val == BLEND_MODES[2] => Ok(BlendMode::Overlay),
        _ => Err(format!("Nothing matching {}", input)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rgba_from_string() {
        let mut res = rgba_from_string("[0,255,0,0]");

        assert_eq!(res.is_ok() == true, true);

        res = rgba_from_string("[0,255,0,as]");

        assert_eq!(res.is_err() == true, true);
    }

    #[test]
    fn test_get_diff_mode_from_string_valid_inputs() {
        // Test with valid inputs
        assert_eq!(
            get_mode_from_string(DIFF_MODES[0]),
            Ok(DiffMode::MarkWithColor)
        );
        assert_eq!(get_mode_from_string(DIFF_MODES[1]), Ok(DiffMode::LCS));
        assert_eq!(get_mode_from_string(DIFF_MODES[2]), Ok(DiffMode::Blend));
    }

    #[test]
    fn test_get_blend_mode_from_string_valid_inputs() {
        // Test with valid inputs
        assert_eq!(string_into_blend_mode(BLEND_MODES[0]), Ok(BlendMode::BIAS));
        assert_eq!(string_into_blend_mode(BLEND_MODES[1]), Ok(BlendMode::HUE));
        assert_eq!(
            string_into_blend_mode(BLEND_MODES[2]),
            Ok(BlendMode::Overlay)
        );
    }

    #[test]
    fn test_get_mode_from_string_invalid_input() {
        // Test with an invalid input
        let input = "unknown";
        let expected_error = format!("Nothing matching {}", input);
        assert_eq!(get_mode_from_string(input), Err(expected_error));
    }
}
