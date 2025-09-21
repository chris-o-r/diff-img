use image::{DynamicImage, GenericImageView, ImageBuffer, Rgba};

use crate::diff_image_error::DiffImgError;

pub fn highlight_changes_with_color(
    before: &DynamicImage,
    after: &DynamicImage,
    color: Rgba<u8>,
) -> Result<DynamicImage, DiffImgError> {
    if before.dimensions() != after.dimensions() {
        return Err(DiffImgError::ImageDimensionMismatch {
            image1: before.dimensions(),
            image2: after.dimensions(),
        });
    }

    let mut result: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::new(after.width(), after.height());

    before
        .pixels()
        .zip(after.pixels())
        .map(|(a, b)| if !a.2.eq(&b.2) { (a.0, a.1, color) } else { (a.0, a.1, a.2) })
        .for_each(|(x, y, pixel)| {
            result.put_pixel(x, y, pixel);
        });

    Ok(DynamicImage::ImageRgba8(result))
}

    #[cfg(test)]
    mod tests {
        use image::RgbaImage;

        use super::*;

        #[test]
        fn test_highlight_changes_with_color() {
            let image1 = image::open("tests/images/image1.png").unwrap();
            let image2 = image::open("tests/images/image2.png").unwrap();
            let color = Rgba([0, 255, 0, 0]);

            let result = highlight_changes_with_color(&image1, &image2, color);

            assert_eq!(result.is_ok(), result.is_ok());
        }

        #[test]
        fn test_highlight_changes_with_color_dimension_mismatch() {
            // Create two images with different dimensions
            let img1: RgbaImage = ImageBuffer::from_pixel(2, 2, Rgba([0, 0, 0, 255]));
            let img2: RgbaImage = ImageBuffer::from_pixel(3, 2, Rgba([0, 0, 0, 255]));
            let dyn_img1 = DynamicImage::ImageRgba8(img1);
            let dyn_img2 = DynamicImage::ImageRgba8(img2);
            let color = Rgba([255, 0, 0, 255]);

            let result = highlight_changes_with_color(&dyn_img1, &dyn_img2, color);

            match result {
                Err(DiffImgError::ImageDimensionMismatch { image1, image2 }) => {
                    assert_eq!(image1, (2, 2));
                    assert_eq!(image2, (3, 2));
                }
                _ => panic!("Expected ImageDimensionMismatch error"),
            }
        }

        #[test]
        fn test_highlight_changes_with_color_no_difference() {
            // Create two identical images
            let img: RgbaImage = ImageBuffer::from_pixel(2, 2, Rgba([10, 20, 30, 255]));
            let dyn_img1 = DynamicImage::ImageRgba8(img.clone());
            let dyn_img2 = DynamicImage::ImageRgba8(img);
            let color = Rgba([255, 0, 0, 255]);

            let result = highlight_changes_with_color(&dyn_img1, &dyn_img2, color).unwrap();

            // All pixels should remain unchanged
            for (_x, _y, pixel) in result.pixels() {
                assert_eq!(pixel, Rgba([10, 20, 30, 255]));
            }
        }

        #[test]
        fn test_highlight_changes_with_color_with_difference() {
            // Create two images with one pixel difference
            let  img1: RgbaImage = ImageBuffer::from_pixel(2, 2, Rgba([10, 20, 30, 255]));
            let mut img2: RgbaImage = ImageBuffer::from_pixel(2, 2, Rgba([10, 20, 30, 255]));
            img2.put_pixel(1, 1, Rgba([99, 88, 77, 255]));
            let dyn_img1 = DynamicImage::ImageRgba8(img1);
            let dyn_img2 = DynamicImage::ImageRgba8(img2);
            let color = Rgba([255, 0, 0, 255]);

            let result = highlight_changes_with_color(&dyn_img1, &dyn_img2, color).unwrap();

            for (x, y, pixel) in result.pixels() {
                if x == 1 && y == 1 {
                    assert_eq!(pixel, Rgba([255, 0, 0, 255]));
                } else {
                    assert_eq!(pixel, Rgba([10, 20, 30, 255]));
                }
            }
        }

        #[test]
        fn test_highlight_changes_multiple_differences() {
            // Create two images with multiple pixel differences
            let img1: RgbaImage = ImageBuffer::from_pixel(3, 3, Rgba([100, 100, 100, 255]));
            let mut img2: RgbaImage = ImageBuffer::from_pixel(3, 3, Rgba([100, 100, 100, 255]));
            
            // Change several pixels
            img2.put_pixel(0, 0, Rgba([200, 0, 0, 255]));
            img2.put_pixel(1, 1, Rgba([0, 200, 0, 255]));
            img2.put_pixel(2, 2, Rgba([0, 0, 200, 255]));
            
            let dyn_img1 = DynamicImage::ImageRgba8(img1);
            let dyn_img2 = DynamicImage::ImageRgba8(img2);
            let highlight_color = Rgba([255, 255, 0, 255]); // Yellow highlight

            let result = highlight_changes_with_color(&dyn_img1, &dyn_img2, highlight_color).unwrap();

            let changed_pixels = [(0, 0), (1, 1), (2, 2)];
            for (x, y, pixel) in result.pixels() {
                if changed_pixels.contains(&(x, y)) {
                    assert_eq!(pixel, Rgba([255, 255, 0, 255]));
                } else {
                    assert_eq!(pixel, Rgba([100, 100, 100, 255]));
                }
            }
        }

        #[test]
        fn test_highlight_changes_alpha_channel_differences() {
            // Test that alpha channel differences are detected
            let img1: RgbaImage = ImageBuffer::from_pixel(2, 2, Rgba([50, 50, 50, 255]));
            let mut img2: RgbaImage = ImageBuffer::from_pixel(2, 2, Rgba([50, 50, 50, 255]));
            
            // Change only the alpha channel
            img2.put_pixel(0, 0, Rgba([50, 50, 50, 128]));
            
            let dyn_img1 = DynamicImage::ImageRgba8(img1);
            let dyn_img2 = DynamicImage::ImageRgba8(img2);
            let highlight_color = Rgba([255, 0, 255, 255]); // Magenta highlight

            let result = highlight_changes_with_color(&dyn_img1, &dyn_img2, highlight_color).unwrap();

            for (x, y, pixel) in result.pixels() {
                if x == 0 && y == 0 {
                    assert_eq!(pixel, Rgba([255, 0, 255, 255]));
                } else {
                    assert_eq!(pixel, Rgba([50, 50, 50, 255]));
                }
            }
        }

        #[test]
        fn test_highlight_changes_large_image() {
            // Test with a larger image to ensure performance and correctness
            let size = 100;
            let img1: RgbaImage = ImageBuffer::from_pixel(size, size, Rgba([64, 128, 192, 255]));
            let mut img2: RgbaImage = ImageBuffer::from_pixel(size, size, Rgba([64, 128, 192, 255]));
            
            // Change a diagonal line of pixels
            for i in 0..size {
                img2.put_pixel(i, i, Rgba([255, 255, 255, 255]));
            }
            
            let dyn_img1 = DynamicImage::ImageRgba8(img1);
            let dyn_img2 = DynamicImage::ImageRgba8(img2);
            let highlight_color = Rgba([255, 0, 0, 255]);

            let result = highlight_changes_with_color(&dyn_img1, &dyn_img2, highlight_color).unwrap();

            assert_eq!(result.dimensions(), (size, size));
            
            // Check that diagonal pixels are highlighted
            for i in 0..size {
                let pixel = result.get_pixel(i, i);
                assert_eq!(pixel, Rgba([255, 0, 0, 255]));
            }
            
            // Check that at least some non-diagonal pixels are not highlighted
            let corner_pixel = result.get_pixel(0, size - 1);
            assert_eq!(corner_pixel, Rgba([64, 128, 192, 255]));
        }

        #[test]
        fn test_highlight_changes_edge_pixels() {
            // Test highlighting changes at image edges
            let mut img1: RgbaImage = ImageBuffer::from_pixel(4, 4, Rgba([0, 0, 0, 255]));
            let mut img2: RgbaImage = ImageBuffer::from_pixel(4, 4, Rgba([0, 0, 0, 255]));
            
            // Change edge pixels
            img2.put_pixel(0, 0, Rgba([255, 255, 255, 255])); // Top-left corner
            img2.put_pixel(3, 0, Rgba([255, 255, 255, 255])); // Top-right corner
            img2.put_pixel(0, 3, Rgba([255, 255, 255, 255])); // Bottom-left corner
            img2.put_pixel(3, 3, Rgba([255, 255, 255, 255])); // Bottom-right corner
            
            let dyn_img1 = DynamicImage::ImageRgba8(img1);
            let dyn_img2 = DynamicImage::ImageRgba8(img2);
            let highlight_color = Rgba([128, 0, 128, 255]);

            let result = highlight_changes_with_color(&dyn_img1, &dyn_img2, highlight_color).unwrap();

            // Check corner pixels are highlighted
            assert_eq!(result.get_pixel(0, 0), Rgba([128, 0, 128, 255]));
            assert_eq!(result.get_pixel(3, 0), Rgba([128, 0, 128, 255]));
            assert_eq!(result.get_pixel(0, 3), Rgba([128, 0, 128, 255]));
            assert_eq!(result.get_pixel(3, 3), Rgba([128, 0, 128, 255]));
            
            // Check center pixels are not highlighted
            assert_eq!(result.get_pixel(1, 1), Rgba([0, 0, 0, 255]));
            assert_eq!(result.get_pixel(2, 2), Rgba([0, 0, 0, 255]));
        }

        #[test]
        fn test_highlight_changes_different_color_formats() {
            // Test with RGB images (converted from RGBA)
            let img1_rgba: RgbaImage = ImageBuffer::from_pixel(3, 3, Rgba([100, 150, 200, 255]));
            let mut img2_rgba: RgbaImage = ImageBuffer::from_pixel(3, 3, Rgba([100, 150, 200, 255]));
            img2_rgba.put_pixel(1, 1, Rgba([200, 100, 50, 255]));
            
            let dyn_img1 = DynamicImage::ImageRgba8(img1_rgba).to_rgb8();
            let dyn_img2 = DynamicImage::ImageRgba8(img2_rgba).to_rgb8();
            let dyn_img1 = DynamicImage::ImageRgb8(dyn_img1);
            let dyn_img2 = DynamicImage::ImageRgb8(dyn_img2);
            
            let highlight_color = Rgba([0, 255, 0, 255]);

            let result = highlight_changes_with_color(&dyn_img1, &dyn_img2, highlight_color).unwrap();

            // Check that the difference is detected even with RGB format
            assert_eq!(result.get_pixel(1, 1), Rgba([0, 255, 0, 255]));
            assert_eq!(result.get_pixel(0, 0), Rgba([100, 150, 200, 255]));
        }

        #[test]
        fn test_highlight_changes_single_pixel_image() {
            // Edge case: single pixel image
            let img1: RgbaImage = ImageBuffer::from_pixel(1, 1, Rgba([100, 100, 100, 255]));
            let img2: RgbaImage = ImageBuffer::from_pixel(1, 1, Rgba([200, 200, 200, 255]));
            
            let dyn_img1 = DynamicImage::ImageRgba8(img1);
            let dyn_img2 = DynamicImage::ImageRgba8(img2);
            let highlight_color = Rgba([255, 0, 0, 255]);

            let result = highlight_changes_with_color(&dyn_img1, &dyn_img2, highlight_color).unwrap();

            assert_eq!(result.dimensions(), (1, 1));
            assert_eq!(result.get_pixel(0, 0), Rgba([255, 0, 0, 255]));
        }

        #[test]
        fn test_highlight_changes_zero_alpha_highlight() {
            // Test with zero alpha in highlight color (should still work)
            let img1: RgbaImage = ImageBuffer::from_pixel(2, 2, Rgba([50, 50, 50, 255]));
            let mut img2: RgbaImage = ImageBuffer::from_pixel(2, 2, Rgba([50, 50, 50, 255]));
            img2.put_pixel(0, 0, Rgba([100, 100, 100, 255]));
            
            let dyn_img1 = DynamicImage::ImageRgba8(img1);
            let dyn_img2 = DynamicImage::ImageRgba8(img2);
            let highlight_color = Rgba([255, 255, 255, 0]); // Zero alpha

            let result = highlight_changes_with_color(&dyn_img1, &dyn_img2, highlight_color).unwrap();

            assert_eq!(result.get_pixel(0, 0), Rgba([255, 255, 255, 0]));
            assert_eq!(result.get_pixel(1, 1), Rgba([50, 50, 50, 255]));
        }
    }