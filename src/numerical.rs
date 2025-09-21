use image::DynamicImage;

// Return a difference ratio between 0 and 1 for the two images
pub fn calculate_diff_ratio(image1: &DynamicImage, image2: &DynamicImage) -> f64 {
    let image1_raw = get_raw_pixels(image1);
    let image2_raw = get_raw_pixels(image2);

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
    // Always convert to RGB8 to ensure only RGB channels are compared
    image.to_rgb8().into_raw()
}

fn abs_diff(x: u8, y: u8) -> u8 {
    if x > y {
        return x - y;
    }
    y - x
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::{ImageBuffer, Rgba, RgbaImage};

    #[test]
    fn test_abs_diff() {
        assert_eq!(abs_diff(5, 8), 3);
        assert_eq!(abs_diff(8, 5), 3);
        assert_eq!(abs_diff(11, 11), 0);
        assert_eq!(abs_diff(0, 255), 255);
        assert_eq!(abs_diff(255, 0), 255);
        assert_eq!(abs_diff(128, 127), 1);
        assert_eq!(abs_diff(1, 0), 1);
    }

    #[test]
    fn test_calculate_diff_ratio() {
        const EXPECTED_RESULT: f64 = 0.030344018901682257;
        let image1 = image::open("tests/images/image1.png").unwrap();
        let image2 = image::open("tests/images/image2.png").unwrap();
        let result = calculate_diff_ratio(&image1, &image2);
        assert_eq!(result, EXPECTED_RESULT);
    }

    #[test]
    fn test_calculate_diff_ratio_identical_images() {
        let image = image::open("tests/images/image1.png").unwrap();
        let result = calculate_diff_ratio(&image, &image);
        assert_eq!(result, 0.0);
    }

    #[test]
    fn test_calculate_diff_ratio_completely_different() {
        // Create two completely different images (black vs white)
        let black_img: RgbaImage = ImageBuffer::from_pixel(10, 10, Rgba([0, 0, 0, 255]));
        let white_img: RgbaImage = ImageBuffer::from_pixel(10, 10, Rgba([255, 255, 255, 255]));

        let dyn_black = DynamicImage::ImageRgba8(black_img);
        let dyn_white = DynamicImage::ImageRgba8(white_img);

        let result = calculate_diff_ratio(&dyn_black, &dyn_white);

        // Should be close to 1.0 since pixels are maximally different
        // Each pixel has 3 channels (RGB), each with max diff of 255
        // Total possible = 255 * 3 * (10*10) = 765000
        // Actual diff = 255 * 3 * (10*10) = 765000
        // Ratio = 765000 / 765000 = 1.0
        assert!((result - 1.0).abs() < 0.0001);
    }

    #[test]
    fn test_calculate_diff_ratio_single_pixel() {
        // Test with single pixel images
        let pixel1: RgbaImage = ImageBuffer::from_pixel(1, 1, Rgba([100, 150, 200, 255]));
        let pixel2: RgbaImage = ImageBuffer::from_pixel(1, 1, Rgba([150, 100, 100, 255]));

        let dyn_img1 = DynamicImage::ImageRgba8(pixel1);
        let dyn_img2 = DynamicImage::ImageRgba8(pixel2);

        let result = calculate_diff_ratio(&dyn_img1, &dyn_img2);

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

        let result = calculate_diff_ratio(&dyn_img1, &dyn_img2);

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
        let result = calculate_diff_ratio(&dyn_rgba, &dyn_rgb);
        assert!(result < 0.001); // Should be very small difference due to format conversion
    }

    #[test]
    fn test_get_raw_pixels() {
        // Test the internal get_raw_pixels function
        let img: RgbaImage = ImageBuffer::from_pixel(2, 2, Rgba([10, 20, 30, 255]));
        let dyn_img = DynamicImage::ImageRgba8(img);

        let pixels = get_raw_pixels(&dyn_img);

        // Should have 2*2*3 = 12 values (width * height * RGB channels)
        assert_eq!(pixels.len(), 12);

        // All RGB values should be [10, 20, 30] repeated
        for i in (0..pixels.len()).step_by(3) {
            assert_eq!(pixels[i], 10); // R
            assert_eq!(pixels[i + 1], 20); // G
            assert_eq!(pixels[i + 2], 30); // B
        }
    }

    #[test]
    fn test_calculate_diff_ratio_edge_case_dimensions() {
        // Test with edge case dimensions
        let tall_img: RgbaImage = ImageBuffer::from_pixel(1, 100, Rgba([128, 128, 128, 255]));

        let dyn_tall = DynamicImage::ImageRgba8(tall_img);

        // Test identical tall image
        let result = calculate_diff_ratio(&dyn_tall, &dyn_tall);
        assert_eq!(result, 0.0); // Identical image should have 0 difference
    }
}
