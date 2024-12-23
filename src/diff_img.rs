use base64::DecodeError;
use image::{DynamicImage, GenericImageView, ImageBuffer, Pixel, Rgb, RgbImage, Rgba};

mod diff;
mod image_creator;

use diff::*;
use image_creator::*;

pub fn highlight_changes_with_color(
    before: DynamicImage,
    after: DynamicImage,
    color: Rgba<u8>,
) -> Result<DynamicImage, String> {
    let mut result: RgbImage = ImageBuffer::new(before.width(), after.height());

    before
        .pixels()
        .into_iter()
        .zip(after.pixels())
        .map(|(a, b)| if !a.2.eq(&b.2) { (a.0, a.1, color) } else { a })
        .for_each(|(x, y, pixel)| {
            result.put_pixel(x, y, Rgb([pixel[0], pixel[1], pixel[2]]));
        });

    Ok(DynamicImage::ImageRgb8(result))
}

// Return a difference ratio between 0 and 1 for the two images
pub fn calculate_diff_ratio(image1: DynamicImage, image2: DynamicImage) -> f64 {
    use std::u8;

    let image1_raw = get_raw_pixels(&image1);
    let image2_raw = get_raw_pixels(&image2);

    // All color types wrap an 8-bit value for each channel
    let total_possible = (u8::MAX as usize * image1_raw.len()) as f64;

    image1_raw
        .into_iter()
        .zip(image2_raw)
        .map(|(a, b)| abs_diff(a, b) as u64)
        .sum::<u64>() as f64
        / total_possible
}

fn get_raw_pixels(image: &DynamicImage) -> Vec<u8> {
    let mut pixels = Vec::new();

    for pixel in image.pixels() {
        let rgba = pixel.2.to_rgb();
        pixels.push(rgba[0]);
        pixels.push(rgba[1]);
        pixels.push(rgba[2]);
    }
    pixels
}

/// abs(x - y) for u8
fn abs_diff(x: u8, y: u8) -> u8 {
    if x > y {
        return x - y;
    }
    return y - x;
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum BlendMode {
    BIAS,
    HUE,
    Overlay,
}

pub fn blend_images(
    image1: DynamicImage,
    image2: DynamicImage,
    blend_mode: BlendMode,
) -> Result<DynamicImage, String> {
    let mut result: RgbImage = ImageBuffer::new(image1.width(), image2.height());

    image1
        .pixels()
        .into_iter()
        .zip(image2.pixels())
        .map(|(a, b)| {
            blend_pixel(
                (a.0, a.1, a.2.to_rgb()),
                (b.0, b.1, b.2.to_rgb()),
                blend_mode,
            )
        })
        .for_each(|(x, y, pixel)| {
            result.put_pixel(x, y, Rgb([pixel[0], pixel[1], pixel[2]]));
        });

    Ok(DynamicImage::ImageRgb8(result))
}

fn blend_pixel(
    pixel_x: (u32, u32, Rgb<u8>),
    pixel_y: (u32, u32, Rgb<u8>),
    blend_mode: BlendMode,
) -> (u32, u32, Rgba<u8>) {
    let a_rgb = pixel_x.2.to_rgb();
    let b_rgb = pixel_y.2.to_rgb();

    let red_diff = abs_diff(a_rgb[0], b_rgb[0]);
    let green_diff = abs_diff(a_rgb[1], b_rgb[1]);
    let blue_diff = abs_diff(a_rgb[2], b_rgb[2]);

    let total_diff = red_diff as f32 + green_diff as f32 + blue_diff as f32;

    let avg_diff = total_diff / 3.0;

    if avg_diff == 0.0 {
        return (
            pixel_x.0,
            pixel_x.1,
            Rgba([a_rgb[0], a_rgb[1], a_rgb[2], 0]),
        );
    } else {
        match blend_mode {
            BlendMode::BIAS => {
                let red_bias = get_bias_from_diff(red_diff, b_rgb[0], 128);
                let green_bias = get_bias_from_diff(green_diff, b_rgb[1], 0);
                let blue_bias = get_bias_from_diff(blue_diff, b_rgb[2], 128);

                let blended_pixel = blend_rgb_pixels(
                    (a_rgb[0], a_rgb[1], a_rgb[2]),
                    (b_rgb[0], b_rgb[1], b_rgb[2]),
                    (red_bias, green_bias, blue_bias),
                );
                return (
                    pixel_x.0,
                    pixel_x.1,
                    Rgba([blended_pixel.0, blended_pixel.1, blended_pixel.2, 0]),
                );
            }
            BlendMode::HUE => {
                // make more purple
                let blended_pixel = blend_rgb_pixels(
                    (a_rgb[0], a_rgb[1], a_rgb[2]),
                    (b_rgb[0], b_rgb[1], b_rgb[2]),
                    (0.3, -0.3, 0.3),
                );
                return (
                    pixel_x.0,
                    pixel_x.1,
                    Rgba([blended_pixel.0, blended_pixel.1, blended_pixel.2, 0]),
                );
            }
            BlendMode::Overlay => {
                let overlayed_pixel = create_overlayed_pixel(
                    (a_rgb[0], a_rgb[1], a_rgb[2]),
                    (b_rgb[0], b_rgb[1], b_rgb[2]),
                    0.5,
                );

                return (
                    pixel_x.0,
                    pixel_x.1,
                    Rgba([overlayed_pixel.0, overlayed_pixel.1, overlayed_pixel.2, 0]),
                );
            }
        }
    }
}

fn create_overlayed_pixel(
    pixel_x: (u8, u8, u8),
    pixel_y: (u8, u8, u8),
    alpha: f32,
) -> (u8, u8, u8) {
    let (red_x, green_x, blue_x) = pixel_x;
    let (red_y, green_y, blue_y) = pixel_y;

    let r_blended = ((alpha * red_x as f32) + ((1.0 - alpha) * red_y as f32)).min(255.0) as u8;
    let g_blended = ((alpha * green_x as f32) + ((1.0 - alpha) * green_y as f32)).min(255.0) as u8;
    let b_blended = ((alpha * blue_x as f32) + ((1.0 - alpha) * blue_y as f32)).min(255.0) as u8;

    return (r_blended, g_blended, b_blended);
}

// Calculate the bias for a color channel based on the difference between two pixels
fn get_bias_from_diff(diff: u8, current: u8, target: u8) -> f32 {
    let diff = diff as f32;
    let current = current as f32;
    let target = target as f32;

    let bias = diff / current;
    let bias = bias * target;

    bias
}

// Blend two RGB pixels together
fn blend_rgb_pixels(
    pixel_x: (u8, u8, u8),
    pixel_y: (u8, u8, u8),
    rgb_bias: (f32, f32, f32),
) -> (u8, u8, u8) {
    let (red_x, green_x, blue_x) = pixel_x;
    let (red_y, green_y, blue_y) = pixel_y;

    // make the colors more purple for the second image

    let red_y_biased = (red_y as f32 * (1.0 + rgb_bias.0)).min(255.0) as u8;
    let green_y_biased = (green_y as f32 * (1.0 + rgb_bias.1)).min(255.0) as u8;
    let blue_y_biased = (blue_y as f32 * (1.0 + rgb_bias.2)).min(255.0) as u8;

    let out_r = (red_x as f32 + red_y_biased as f32) / 2.0;
    let out_g = (green_x as f32 + green_y_biased as f32) / 2.0;
    let out_b = (blue_x as f32 + blue_y_biased as f32) / 2.0;

    // Return the blended pixel, clamping each value to [0, 255]
    (
        out_r.min(255.0).max(0.0) as u8,
        out_g.min(255.0).max(0.0) as u8,
        out_b.min(255.0).max(0.0) as u8,
    )
}

pub fn lcs_diff(
    before: &mut DynamicImage,
    after: &mut DynamicImage,
    rate: f32,
) -> Result<DynamicImage, DecodeError> {
    let compare_before = CompareImage::new(
        before.dimensions(),
        before.pixels().map(|pix| pix.2).collect(),
    );
    let compare_after = CompareImage::new(
        after.dimensions(),
        after.pixels().map(|pix| pix.2).collect(),
    );
    let result = diff(compare_before, compare_after);

    let mut added: Vec<usize> = Vec::new();
    let mut removed: Vec<usize> = Vec::new();
    for d in result.iter() {
        match d {
            &lcs_diff::DiffResult::Added(ref a) => added.push(a.new_index.unwrap()),
            &lcs_diff::DiffResult::Removed(ref r) => removed.push(r.old_index.unwrap()),
            _ => (),
        }
    }

    mark_org_image(before, RED, rate, &removed);
    mark_org_image(after, GREEN, rate, &added);

    get_diff_image(before.dimensions().0, after.dimensions().0, &result, rate)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_highlight_changes_with_color() {
        let image1 = image::open("tests/images/image1.png").unwrap();
        let image2 = image::open("tests/images/image2.png").unwrap();
        let color = Rgba([0, 255, 0, 0]);

        let result = highlight_changes_with_color(image1, image2, color);

        assert_eq!(result.is_ok(), result.is_ok());
    }

    #[test]
    fn test_calculate_diff_ratio() {
        const EXPECTED_RESULT: f64 = 0.030344018901682257;
        let image1 = image::open("tests/images/image1.png").unwrap();
        let image2 = image::open("tests/images/image2.png").unwrap();
        let result = calculate_diff_ratio(image1, image2);
        assert_eq!(result, EXPECTED_RESULT);
    }

    #[test]
    fn test_create_overlayed_pixel() {
        let pixel_x = (100, 150, 200);
        let pixel_y = (50, 100, 150);
        let mut alpha = 0.0;
        let mut result = create_overlayed_pixel(pixel_x, pixel_y, alpha);

        assert_eq!(result, pixel_y);

        alpha = 1.0;
        result = create_overlayed_pixel(pixel_x, pixel_y, alpha);

        assert_eq!(result, pixel_x);

        alpha = 0.5;
        result = create_overlayed_pixel(pixel_x, pixel_y, alpha);

        assert_eq!(result, (75, 125, 175));
    }

    #[test]
    fn test_blend_rgb_pixels_no_bias() {
        let pixel_x = (100, 150, 200);
        let pixel_y = (50, 100, 150);
        let rgb_bias = (0.0, 0.0, 0.0);
        let result = blend_rgb_pixels(pixel_x, pixel_y, rgb_bias);
        assert_eq!(result, (75, 125, 175)); // Calculated expected result with no bias
    }

    #[test]
    fn test_blend_rgb_pixels_min_values() {
        let pixel_x = (0, 0, 0);
        let pixel_y = (0, 0, 0);
        let rgb_bias = (0.1, 0.1, 0.1);
        let result = blend_rgb_pixels(pixel_x, pixel_y, rgb_bias);
        assert_eq!(result, (0, 0, 0)); // Both pixels are black, result should be black
    }

    #[test]
    fn test_blend_rgb_pixels_max_values() {
        let pixel_x = (255, 255, 255);
        let pixel_y = (255, 255, 255);
        let rgb_bias = (0.1, 0.1, 0.1);
        let result = blend_rgb_pixels(pixel_x, pixel_y, rgb_bias);
        assert_eq!(result, (255, 255, 255)); // Both pixels are white, result should be white
    }

    #[test]
    fn test_blend_rgb_pixels_bias_clamping() {
        let pixel_x = (0, 0, 0);
        let pixel_y = (255, 255, 255);
        let rgb_bias = (10.0, 10.0, 10.0); // High bias to test clamping
        let result = blend_rgb_pixels(pixel_x, pixel_y, rgb_bias);
        assert_eq!(result, (127, 127, 127)); // pixel_y will be clamped to (255, 255, 255), average is (127.5, 127.5, 127.5) -> (127, 127, 127)
    }

    #[test]
    fn test_get_diff_from_images_overlay() {
        let image1 = image::open("tests/images/image1.png").unwrap();
        let image2 = image::open("tests/images/image2.png").unwrap();
        let blend_mode = BlendMode::Overlay;
        let result = blend_images(image1, image2, blend_mode);
        assert_eq!(result.is_ok(), true);
    }
}
