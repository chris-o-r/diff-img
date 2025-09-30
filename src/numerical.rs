use image::{DynamicImage, GenericImageView};

use crate::diff_image_error::DiffImgError;

// Return a difference ratio between 0 and 1 for the two images
pub fn calculate_diff_ratio(
    image1: &DynamicImage,
    image2: &DynamicImage,
) -> Result<f64, DiffImgError> {
    const MAX_PIXELS: u64 = 255;

    if image1.dimensions() != image2.dimensions() {
        return Err(DiffImgError::ImageDimensionMismatch {
            image1: image1.dimensions(),
            image2: image2.dimensions(),
        });
    }

    let image1_raw = image1.to_rgb8().into_raw();
    let image2_raw = image2.to_rgb8().into_raw();

    let total_possible = (image1_raw.len() as u64) * MAX_PIXELS;

    let total_diff: u64 = image1_raw
        .into_iter()
        .zip(image2_raw)
        .map(|(a, b)| (a as i32 - b as i32).abs() as u64)
        .sum();

    Ok(total_diff as f64 / total_possible as f64)
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::{ImageBuffer, Rgba, RgbaImage};

    #[test]
    fn test_calculate_diff_ratio() {
        const EXPECTED_RESULT: f64 = 1.0;
        const WIDTH: u32 = 100;
        const HEIGHT: u32 = 100;

        let image1 =
            DynamicImage::ImageRgba8(ImageBuffer::from_pixel(WIDTH, HEIGHT, Rgba([0, 0, 0, 255])));
        let image2 = DynamicImage::ImageRgba8(ImageBuffer::from_pixel(
            WIDTH,
            HEIGHT,
            Rgba([255, 255, 255, 255]),
        ));
        let result = calculate_diff_ratio(&image1, &image2).unwrap();
        assert_eq!(result, EXPECTED_RESULT);
    }

    #[test]
    fn test_calculate_diff_ratio_identical_images() {
        let image =
            DynamicImage::ImageRgba8(ImageBuffer::from_pixel(100, 100, Rgba([0, 0, 0, 255])));
        let result = calculate_diff_ratio(&image, &image).unwrap();
        assert_eq!(result, 0.0);
    }

    #[test]
    fn test_calculate_diff_ratio_invalid_size() {
        let image1 =
            DynamicImage::ImageRgba8(ImageBuffer::from_pixel(100, 100, Rgba([0, 0, 0, 255])));
        let image2 = DynamicImage::ImageRgba8(ImageBuffer::from_pixel(
            100,
            101,
            Rgba([255, 255, 255, 255]),
        ));
        let result = calculate_diff_ratio(&image1, &image2);

        match result {
            Err(DiffImgError::ImageDimensionMismatch { image1, image2 }) => {
                assert_eq!(image1, (100, 100));
                assert_eq!(image2, (100, 101));
            }
            _ => panic!("Expected ImageDimensionMismatch error"),
        }
    }

    #[test]
    fn test_calculate_diff_ratio_single_pixel() {
        // Test with single pixel images
        let pixel1: RgbaImage = ImageBuffer::from_pixel(1, 1, Rgba([100, 150, 200, 255]));
        let pixel2: RgbaImage = ImageBuffer::from_pixel(1, 1, Rgba([150, 100, 100, 255]));

        let dyn_img1 = DynamicImage::ImageRgba8(pixel1);
        let dyn_img2 = DynamicImage::ImageRgba8(pixel2);

        let result = calculate_diff_ratio(&dyn_img1, &dyn_img2).unwrap();

        // Manual calculation: |100-150| + |150-100| + |200-100| = 50 + 50 + 100 = 200
        // Total possible = 255 * 3 = 765
        // Expected ratio = 200 / 765 ≈ 0.2614
        assert!((result - 0.26143790849673204).abs() < 0.0001);
    }

    #[test]
    fn test_calculate_diff_ratio_partial_difference() {
        // Create images where only some pixels differ
        let img1: RgbaImage = ImageBuffer::from_pixel(3, 3, Rgba([100, 100, 100, 255]));
        let mut img2: RgbaImage = ImageBuffer::from_pixel(3, 3, Rgba([100, 100, 100, 255]));

        // Change only the center pixel
        img2.put_pixel(1, 1, Rgba([200, 200, 200, 255]));

        let dyn_img1 = DynamicImage::ImageRgba8(img1);
        let dyn_img2 = DynamicImage::ImageRgba8(img2);

        let result = calculate_diff_ratio(&dyn_img1, &dyn_img2).unwrap();

        // Only 1 pixel out of 9 differs, by 100 in each of 3 channels = 300 total
        // Total possible = 255 * 3 * 9 = 6885
        // Expected ratio = 300 / 6885 ≈ 0.04357
        assert!((result - 0.04357298474945534).abs() < 0.0001);
    }

    #[test]
    fn test_calculate_diff_ratio_different_formats() {
        // Test with different image formats (RGB vs RGBA)
        let rgba_img: RgbaImage = ImageBuffer::from_pixel(5, 5, Rgba([128, 64, 192, 255]));
        let dyn_rgba = DynamicImage::ImageRgba8(rgba_img);
        let dyn_rgb = dyn_rgba.to_rgb8();
        let dyn_rgb = DynamicImage::ImageRgb8(dyn_rgb);

        // Since we're only extracting RGB channels, they should be very similar
        let result = calculate_diff_ratio(&dyn_rgba, &dyn_rgb).unwrap();
        assert!(result < 0.001); // Should be very small difference due to format conversion
    }

    #[test]
    fn test_calculate_diff_ratio_edge_case_dimensions() {
        // Test with edge case dimensions
        let tall_img: RgbaImage = ImageBuffer::from_pixel(1, 100, Rgba([128, 128, 128, 255]));

        let dyn_tall = DynamicImage::ImageRgba8(tall_img);

        // Test identical tall image
        let result = calculate_diff_ratio(&dyn_tall, &dyn_tall).unwrap();
        assert_eq!(result, 0.0); // Identical image should have 0 difference
    }
}
