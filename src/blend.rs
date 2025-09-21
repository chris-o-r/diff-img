use image::{DynamicImage, GenericImageView, ImageBuffer, Pixel, Rgb, RgbImage, Rgba};

use crate::diff_image_error::DiffImgError;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum BlendMode {
    BIAS,
    HUE,
    Overlay,
}

pub fn blend_images(
    image1: &DynamicImage,
    image2: &DynamicImage,
    blend_mode: BlendMode,
) -> Result<DynamicImage, DiffImgError> {
    if image1.width() != image2.width() || image1.height() != image2.height() {
        return Err(DiffImgError::ImageDimensionMismatch {
            image1: (image1.width(), image1.height()),
            image2: (image2.width(), image2.height()),
        });
    }
    let mut result: RgbImage = ImageBuffer::new(image1.width(), image2.height());

    image1
        .pixels()
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
        (
            pixel_x.0,
            pixel_x.1,
            Rgba([a_rgb[0], a_rgb[1], a_rgb[2], 0]),
        )
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
                (
                    pixel_x.0,
                    pixel_x.1,
                    Rgba([blended_pixel.0, blended_pixel.1, blended_pixel.2, 0]),
                )
            }
            BlendMode::HUE => {
                // make more purple
                let blended_pixel = blend_rgb_pixels(
                    (a_rgb[0], a_rgb[1], a_rgb[2]),
                    (b_rgb[0], b_rgb[1], b_rgb[2]),
                    (0.3, -0.3, 0.3),
                );
                (
                    pixel_x.0,
                    pixel_x.1,
                    Rgba([blended_pixel.0, blended_pixel.1, blended_pixel.2, 0]),
                )
            }
            BlendMode::Overlay => {
                let overlayed_pixel = create_overlayed_pixel(
                    (a_rgb[0], a_rgb[1], a_rgb[2]),
                    (b_rgb[0], b_rgb[1], b_rgb[2]),
                    0.5,
                );

                (
                    pixel_x.0,
                    pixel_x.1,
                    Rgba([overlayed_pixel.0, overlayed_pixel.1, overlayed_pixel.2, 0]),
                )
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

    (r_blended, g_blended, b_blended)
}

// Calculate the bias for a color channel based on the difference between two pixels
fn get_bias_from_diff(diff: u8, current: u8, target: u8) -> f32 {
    let diff = diff as f32;
    let current = current as f32;
    let target = target as f32;

    if diff == 0.0 || current == 0.0 {
        return 0.0;
    }

    let bias = diff / current;

    bias * target
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
        out_r.clamp(0.0, 255.0) as u8,
        out_g.clamp(0.0, 255.0) as u8,
        out_b.clamp(0.0, 255.0) as u8,
    )
}

/// abs(x - y) for u8
fn abs_diff(x: u8, y: u8) -> u8 {
    if x > y {
        return x - y;
    }
    y - x
}

#[cfg(test)]
mod tests {
    use super::*;

    // Helper function to create a simple test image
    fn create_test_image(width: u32, height: u32, color: [u8; 3]) -> DynamicImage {
        let img: RgbImage = ImageBuffer::from_fn(width, height, |_, _| Rgb(color));
        DynamicImage::ImageRgb8(img)
    }

    #[test]
    fn test_abs_diff() {
        assert_eq!(abs_diff(5, 8), 3);
        assert_eq!(abs_diff(8, 5), 3);
        assert_eq!(abs_diff(11, 11), 0);
        assert_eq!(abs_diff(0, 255), 255);
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
    fn test_blend_dimension_mismatch() {
        let img1 = create_test_image(10, 10, [255, 0, 0]);
        let img2 = create_test_image(8, 10, [0, 255, 0]); // Different width

        let result = blend_images(&img1, &img2, BlendMode::BIAS);
        assert!(result.is_err());

        if let Err(err) = result {
            match err {
                DiffImgError::ImageDimensionMismatch { image1, image2 } => {
                    assert_eq!(image1, (10, 10));
                    assert_eq!(image2, (8, 10));
                }
                _ => panic!("Expected ImageDimensionMismatch error"),
            }
        }
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
    fn test_blend_images_overlay() {
        let image1 = create_test_image(10, 10, [255, 0, 0]); // Red
        let image2 = create_test_image(10, 10, [0, 255, 0]); // Green
        let blend_mode = BlendMode::Overlay;
        let result = blend_images(&image1, &image2, blend_mode);
        assert!(result.is_ok());

        let blended = result.unwrap();
        assert_eq!(blended.dimensions(), (10, 10));
    }

    #[test]
    fn test_blend_images_bias() {
        let image1 = create_test_image(5, 5, [128, 128, 128]); // Gray
        let image2 = create_test_image(5, 5, [64, 64, 64]); // Dark gray
        let blend_mode = BlendMode::BIAS;
        let result = blend_images(&image1, &image2, blend_mode);
        assert!(result.is_ok());

        let blended = result.unwrap();
        assert_eq!(blended.dimensions(), (5, 5));
    }

    #[test]
    fn test_blend_images_hue() {
        let image1 = create_test_image(8, 8, [200, 100, 50]);
        let image2 = create_test_image(8, 8, [50, 200, 100]);
        let blend_mode = BlendMode::HUE;
        let result = blend_images(&image1, &image2, blend_mode);
        assert!(result.is_ok());

        let blended = result.unwrap();
        assert_eq!(blended.dimensions(), (8, 8));
    }

    #[test]
    fn test_blend_mode_equality() {
        assert_eq!(BlendMode::BIAS, BlendMode::BIAS);
        assert_eq!(BlendMode::HUE, BlendMode::HUE);
        assert_eq!(BlendMode::Overlay, BlendMode::Overlay);
        assert_ne!(BlendMode::BIAS, BlendMode::HUE);
    }

    #[test]
    fn test_get_bias_from_diff() {
        let bias = get_bias_from_diff(100, 200, 128);
        assert_eq!(bias, 64.0); // (100/200) * 128 = 0.5 * 128 = 64

        let bias_zero = get_bias_from_diff(0, 255, 128);
        assert_eq!(bias_zero, 0.0); // No difference, no bias
    }

    #[test]
    fn test_blend_images_identical() {
        let image1 = create_test_image(6, 6, [100, 150, 200]);
        let image2 = create_test_image(6, 6, [100, 150, 200]); // Identical
        let blend_mode = BlendMode::Overlay;
        let result = blend_images(&image1, &image2, blend_mode);
        assert!(result.is_ok());

        let blended = result.unwrap();
        assert_eq!(blended.dimensions(), (6, 6));
    }

    #[test]
    fn test_create_overlayed_pixel_edge_cases() {
        // Test with extreme alpha values
        let pixel_x = (255, 255, 255);
        let pixel_y = (0, 0, 0);

        // Alpha = 0 should return pixel_y
        let result_0 = create_overlayed_pixel(pixel_x, pixel_y, 0.0);
        assert_eq!(result_0, pixel_y);

        // Alpha = 1 should return pixel_x
        let result_1 = create_overlayed_pixel(pixel_x, pixel_y, 1.0);
        assert_eq!(result_1, pixel_x);

        // Alpha = 0.25 should be closer to pixel_y
        let result_025 = create_overlayed_pixel(pixel_x, pixel_y, 0.25);
        assert_eq!(result_025, (63, 63, 63)); // 0.25*255 + 0.75*0 = 63.75 -> 63
    }
}
