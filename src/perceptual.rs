use image::{DynamicImage, GenericImageView, ImageBuffer, Rgb, Rgba};

use crate::diff_image_error::DiffImgError;

// ...existing code...

// Create a visual representation of perceptual differences
pub fn create_perceptual_diff_image(
    imga: &DynamicImage,
    imgb: &DynamicImage,
    threshold: f32,
) -> Result<DynamicImage, DiffImgError> {
    let differences = perceptual_diff(imga, imgb)?;
    let (width, height) = imga.dimensions();
    
    // Create a new image buffer
    let mut result_image = ImageBuffer::new(width, height);
    
    // Start with the original image as base
    for (x, y, pixel) in imga.pixels() {
        let rgb = Rgb([pixel[0], pixel[1], pixel[2]]);
        result_image.put_pixel(x, y, rgb);
    }
    
    // Highlight areas with perceptual differences
    for &(x, y, diff_intensity) in &differences {
        if diff_intensity > threshold {
            // Color intensity based on difference magnitude
            let intensity = (diff_intensity / 10.0).min(1.0); // Normalize to 0-1
            let highlight_color = get_diff_color(intensity);
            result_image.put_pixel(x, y, highlight_color);
        }
    }
    
    Ok(DynamicImage::ImageRgb8(result_image))
}

// Create a heatmap showing perceptual difference intensity
pub fn create_perceptual_heatmap(
    imga: &DynamicImage,
    imgb: &DynamicImage,
) -> Result<DynamicImage, DiffImgError> {
    let differences = perceptual_diff(imga, imgb)?;
    let (width, height) = imga.dimensions();
    
    // Create heatmap lookup for fast access
    let mut heatmap = vec![vec![0.0f32; width as usize]; height as usize];
    let mut max_diff = 0.0f32;
    
    // Fill heatmap and find maximum difference
    for &(x, y, diff_intensity) in &differences {
        heatmap[y as usize][x as usize] = diff_intensity;
        max_diff = max_diff.max(diff_intensity);
    }
    
    // Create the heatmap image
    let mut result_image = ImageBuffer::new(width, height);
    
    for y in 0..height {
        for x in 0..width {
            let diff_value = heatmap[y as usize][x as usize];
            let normalized_diff = if max_diff > 0.0 { diff_value / max_diff } else { 0.0 };
            let heatmap_color = create_heatmap_color(normalized_diff);
            result_image.put_pixel(x, y, heatmap_color);
        }
    }
    
    Ok(DynamicImage::ImageRgb8(result_image))
}

// Create a side-by-side comparison with overlay
pub fn create_perceptual_comparison(
    imga: &DynamicImage,
    imgb: &DynamicImage,
    overlay_alpha: f32,
) -> Result<DynamicImage, DiffImgError> {
    let differences = perceptual_diff(imga, imgb)?;
    let (width, height) = imga.dimensions();
    
    // Create side-by-side image (twice the width)
    let mut result_image = ImageBuffer::new(width * 2, height);
    
    // Left side: original image A
    for (x, y, pixel) in imga.pixels() {
        let rgb = Rgb([pixel[0], pixel[1], pixel[2]]);
        result_image.put_pixel(x, y, rgb);
    }
    
    // Right side: image B with difference overlay
    for (x, y, pixel) in imgb.pixels() {
        let rgb = Rgb([pixel[0], pixel[1], pixel[2]]);
        result_image.put_pixel(x + width, y, rgb);
    }
    
    // Add difference overlay to both sides
    for &(x, y, diff_intensity) in &differences {
        if diff_intensity > 2.3 { // JND threshold
            let intensity = (diff_intensity / 10.0).min(1.0);
            let overlay_color = get_diff_color(intensity);
            
            // Blend overlay with original pixels
            let left_pixel = result_image.get_pixel(x, y);
            let right_pixel = result_image.get_pixel(x + width, y);
            
            let blended_left = blend_with_overlay(*left_pixel, overlay_color, overlay_alpha);
            let blended_right = blend_with_overlay(*right_pixel, overlay_color, overlay_alpha);
            
            result_image.put_pixel(x, y, blended_left);
            result_image.put_pixel(x + width, y, blended_right);
        }
    }
    
    Ok(DynamicImage::ImageRgb8(result_image))
}

// Create difference-only image (black background with colored differences)
pub fn create_perceptual_diff_only(
    imga: &DynamicImage,
    imgb: &DynamicImage,
    threshold: f32,
) -> Result<DynamicImage, DiffImgError> {
    let differences = perceptual_diff(imga, imgb)?;
    let (width, height) = imga.dimensions();
    
    // Start with black background
    let mut result_image = ImageBuffer::from_pixel(width, height, Rgb([0, 0, 0]));
    
    // Only show the differences
    for &(x, y, diff_intensity) in &differences {
        if diff_intensity > threshold {
            let intensity = (diff_intensity / 10.0).min(1.0);
            let diff_color = get_diff_color(intensity);
            result_image.put_pixel(x, y, diff_color);
        }
    }
    
    Ok(DynamicImage::ImageRgb8(result_image))
}

// Helper functions for color mapping
fn get_diff_color(intensity: f32) -> Rgb<u8> {
    // Red intensity based on difference magnitude
    let red = (255.0 * intensity) as u8;
    Rgb([red, 0, 0])
}

fn create_heatmap_color(normalized_value: f32) -> Rgb<u8> {
    // Create a blue -> green -> yellow -> red heatmap
    if normalized_value < 0.25 {
        // Blue to cyan
        let t = normalized_value * 4.0;
        Rgb([0, (255.0 * t) as u8, 255])
    } else if normalized_value < 0.5 {
        // Cyan to green
        let t = (normalized_value - 0.25) * 4.0;
        Rgb([0, 255, (255.0 * (1.0 - t)) as u8])
    } else if normalized_value < 0.75 {
        // Green to yellow
        let t = (normalized_value - 0.5) * 4.0;
        Rgb([(255.0 * t) as u8, 255, 0])
    } else {
        // Yellow to red
        let t = (normalized_value - 0.75) * 4.0;
        Rgb([255, (255.0 * (1.0 - t)) as u8, 0])
    }
}

fn blend_with_overlay(base: Rgb<u8>, overlay: Rgb<u8>, alpha: f32) -> Rgb<u8> {
    let r = (base[0] as f32 * (1.0 - alpha) + overlay[0] as f32 * alpha) as u8;
    let g = (base[1] as f32 * (1.0 - alpha) + overlay[1] as f32 * alpha) as u8;
    let b = (base[2] as f32 * (1.0 - alpha) + overlay[2] as f32 * alpha) as u8;
    Rgb([r, g, b])
}

// Fix the perceptual_diff function return statement
pub fn perceptual_diff(imga: &DynamicImage, imgb: &DynamicImage) -> Result<Vec<(u32, u32, f32)>, DiffImgError> {
    let mut differences = Vec::new();
    
    if imga.dimensions() != imgb.dimensions() {
        return Err(DiffImgError::ImageDimensionMismatch { 
            image1: imga.dimensions(), 
            image2: imgb.dimensions() 
        });
    }
    
    let width = imga.dimensions().0;
    for (i, (pixel_a, pixel_b)) in imga.pixels().zip(imgb.pixels()).enumerate() {
        // pixel_a and pixel_b are (x, y, rgba) tuples, we want the rgba part (index 2)
        let diff = delta_e_distance(&pixel_a.2, &pixel_b.2);
        if diff > 2.3 { // JND (Just Noticeable Difference) threshold
            let x = i as u32 % width;
            let y = i as u32 / width;
            differences.push((x, y, diff));
        }
    }
    Ok(differences) // Fixed: was missing Ok()
}

// Delta E distance function for perceptual color difference
fn delta_e_distance(pixel_a: &Rgba<u8>, pixel_b: &Rgba<u8>) -> f32 {
    // Simplified Delta E calculation using RGB to LAB conversion
    let lab_a = rgb_to_lab(pixel_a[0], pixel_a[1], pixel_a[2]);
    let lab_b = rgb_to_lab(pixel_b[0], pixel_b[1], pixel_b[2]);
    
    let dl = lab_a.0 - lab_b.0;
    let da = lab_a.1 - lab_b.1;
    let db = lab_a.2 - lab_b.2;
    
    (dl * dl + da * da + db * db).sqrt()
}

// Add missing rgb_to_lab function
fn rgb_to_lab(r: u8, g: u8, b: u8) -> (f32, f32, f32) {
    // Convert RGB to LAB color space for perceptual comparison
    let r = r as f32 / 255.0;
    let g = g as f32 / 255.0;
    let b = b as f32 / 255.0;
    
    // Simplified RGB to LAB conversion
    let l = 0.299 * r + 0.587 * g + 0.114 * b;
    let a = 0.5 * (r - g);
    let bb = 0.5 * (g - b);
    
    (l * 100.0, a * 100.0, bb * 100.0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::{ImageBuffer, RgbImage};

    // Helper function to create a simple test image
    fn create_test_image(width: u32, height: u32, color: [u8; 3]) -> DynamicImage {
        let img: RgbImage = ImageBuffer::from_fn(width, height, |_, _| {
            Rgb(color)
        });
        DynamicImage::ImageRgb8(img)
    }

    // Helper function to create a test image with a different region
    fn create_test_image_with_region(width: u32, height: u32, base_color: [u8; 3], region_color: [u8; 3]) -> DynamicImage {
        let img: RgbImage = ImageBuffer::from_fn(width, height, |x, y| {
            if x >= 10 && x < 20 && y >= 10 && y < 20 {
                Rgb(region_color)
            } else {
                Rgb(base_color)
            }
        });
        DynamicImage::ImageRgb8(img)
    }

    #[test]
    fn test_rgb_to_lab_black() {
        let result = rgb_to_lab(0, 0, 0);
        assert_eq!(result.0, 0.0); // L should be 0 for black
        assert_eq!(result.1, 0.0); // a should be 0 for black
        assert_eq!(result.2, 0.0); // b should be 0 for black
    }

    #[test]
    fn test_rgb_to_lab_white() {
        let result = rgb_to_lab(255, 255, 255);
        assert_eq!(result.0, 100.0); // L should be 100 for white
        assert_eq!(result.1, 0.0);   // a should be 0 for white
        assert_eq!(result.2, 0.0);   // b should be 0 for white
    }

    #[test]
    fn test_rgb_to_lab_red() {
        let result = rgb_to_lab(255, 0, 0);
        assert_eq!(result.0, 29.9); // L component for red
        assert_eq!(result.1, 50.0); // a component for red (positive = more red)
        assert_eq!(result.2, 0.0);  // b component for red
    }

    #[test]
    fn test_delta_e_distance_identical_pixels() {
        let pixel_a = Rgba([255, 128, 64, 255]);
        let pixel_b = Rgba([255, 128, 64, 255]);
        let distance = delta_e_distance(&pixel_a, &pixel_b);
        assert_eq!(distance, 0.0);
    }

    #[test]
    fn test_delta_e_distance_different_pixels() {
        let pixel_a = Rgba([255, 0, 0, 255]); // Red
        let pixel_b = Rgba([0, 255, 0, 255]); // Green
        let distance = delta_e_distance(&pixel_a, &pixel_b);
        assert!(distance > 0.0);
        assert!(distance > 50.0); // Should be a significant perceptual difference
    }

    #[test]
    fn test_delta_e_distance_small_difference() {
        let pixel_a = Rgba([255, 255, 255, 255]); // White
        let pixel_b = Rgba([254, 254, 254, 255]); // Very light gray
        let distance = delta_e_distance(&pixel_a, &pixel_b);
        assert!(distance > 0.0);
        assert!(distance < 5.0); // Should be a small perceptual difference
    }

    #[test]
    fn test_perceptual_diff_identical_images() {
        let img1 = create_test_image(50, 50, [255, 128, 64]);
        let img2 = create_test_image(50, 50, [255, 128, 64]);
        
        let result = perceptual_diff(&img1, &img2).unwrap();
        assert_eq!(result.len(), 0); // No differences should be found
    }

    #[test]
    fn test_perceptual_diff_different_images() {
        let img1 = create_test_image(50, 50, [255, 0, 0]); // Red
        let img2 = create_test_image(50, 50, [0, 255, 0]); // Green
        
        let result = perceptual_diff(&img1, &img2).unwrap();
        assert!(result.len() > 0); // Should find many differences
        assert_eq!(result.len(), 50 * 50); // All pixels should be different
    }

    #[test]
    fn test_perceptual_diff_dimension_mismatch() {
        let img1 = create_test_image(50, 50, [255, 128, 64]);
        let img2 = create_test_image(60, 50, [255, 128, 64]);
        
        let result = perceptual_diff(&img1, &img2);
        assert!(result.is_err());
        match result {
            Err(DiffImgError::ImageDimensionMismatch { image1, image2 }) => {
                assert_eq!(image1, (50, 50));
                assert_eq!(image2, (60, 50));
            },
            _ => panic!("Expected ImageDimensionMismatch error"),
        }
    }

    #[test]
    fn test_perceptual_diff_with_region() {
        let img1 = create_test_image_with_region(50, 50, [128, 128, 128], [255, 0, 0]); // Gray with red region
        let img2 = create_test_image_with_region(50, 50, [128, 128, 128], [0, 255, 0]); // Gray with green region
        
        let result = perceptual_diff(&img1, &img2).unwrap();
        
        // Should only find differences in the 10x10 region (from 10,10 to 20,20)
        assert!(result.len() > 0);
        assert!(result.len() <= 100); // At most 100 different pixels (10x10 region)
        
        // Check that all differences are within the expected region
        for &(x, y, _diff) in &result {
            assert!(x >= 10 && x < 20);
            assert!(y >= 10 && y < 20);
        }
    }

    #[test]
    fn test_create_perceptual_diff_image() {
        let img1 = create_test_image(20, 20, [255, 0, 0]); // Red
        let img2 = create_test_image(20, 20, [0, 255, 0]); // Green
        
        let result = create_perceptual_diff_image(&img1, &img2, 2.0).unwrap();
        assert_eq!(result.dimensions(), (20, 20));
    }

    #[test]
    fn test_create_perceptual_heatmap() {
        let img1 = create_test_image_with_region(30, 30, [128, 128, 128], [255, 0, 0]);
        let img2 = create_test_image_with_region(30, 30, [128, 128, 128], [0, 255, 0]);
        
        let result = create_perceptual_heatmap(&img1, &img2).unwrap();
        assert_eq!(result.dimensions(), (30, 30));
    }

    #[test]
    fn test_create_perceptual_comparison() {
        let img1 = create_test_image(25, 25, [255, 0, 0]);
        let img2 = create_test_image(25, 25, [0, 255, 0]);
        
        let result = create_perceptual_comparison(&img1, &img2, 0.5).unwrap();
        assert_eq!(result.dimensions(), (50, 25)); // Should be twice the width
    }

    #[test]
    fn test_create_perceptual_diff_only() {
        let img1 = create_test_image_with_region(20, 20, [128, 128, 128], [255, 0, 0]);
        let img2 = create_test_image_with_region(20, 20, [128, 128, 128], [0, 255, 0]);
        
        let result = create_perceptual_diff_only(&img1, &img2, 2.0).unwrap();
        assert_eq!(result.dimensions(), (20, 20));
    }

    #[test]
    fn test_get_diff_color() {
        let color_low = get_diff_color(0.0);
        let color_high = get_diff_color(1.0);
        
        assert_eq!(color_low, Rgb([0, 0, 0])); // Should be black for no intensity
        assert_eq!(color_high, Rgb([255, 0, 0])); // Should be full red for max intensity
    }

    #[test]
    fn test_create_heatmap_color() {
        let color_blue = create_heatmap_color(0.0);   // Should be blue
        let color_green = create_heatmap_color(0.5);  // Should be green-ish
        let color_red = create_heatmap_color(1.0);    // Should be red
        
        // Blue region (0.0 - 0.25)
        assert_eq!(color_blue, Rgb([0, 0, 255]));
        
        // Red region (0.75 - 1.0)
        assert_eq!(color_red, Rgb([255, 0, 0]));
        
        // Green region should have high green component
        assert!(color_green[1] > 200); // High green component
    }

    #[test]
    fn test_blend_with_overlay() {
        let base = Rgb([128, 128, 128]); // Gray
        let overlay = Rgb([255, 0, 0]);  // Red
        
        let blended_50 = blend_with_overlay(base, overlay, 0.5);
        let blended_0 = blend_with_overlay(base, overlay, 0.0);
        let blended_100 = blend_with_overlay(base, overlay, 1.0);
        
        // 50% blend should be halfway between base and overlay
        assert_eq!(blended_50, Rgb([191, 64, 64])); // (128*0.5 + 255*0.5, 128*0.5 + 0*0.5, 128*0.5 + 0*0.5)
        
        // 0% overlay should be pure base
        assert_eq!(blended_0, base);
        
        // 100% overlay should be pure overlay
        assert_eq!(blended_100, overlay);
    }

    #[test]
    fn test_perceptual_threshold_sensitivity() {
        let img1 = create_test_image(10, 10, [255, 255, 255]); // White
        let img2 = create_test_image(10, 10, [254, 254, 254]); // Very light gray
        
        let differences = perceptual_diff(&img1, &img2).unwrap();
        
        // With default threshold of 2.3, very small differences should not be detected
        assert_eq!(differences.len(), 0);
    }

    #[test]
    fn test_perceptual_diff_coordinates() {
        // Create images with known different regions to test coordinate accuracy
        let img1 = create_test_image_with_region(10, 10, [128, 128, 128], [255, 0, 0]);
        let img2 = create_test_image_with_region(10, 10, [128, 128, 128], [0, 255, 0]);
        
        let differences = perceptual_diff(&img1, &img2).unwrap();
        
        // Verify that reported coordinates are within image bounds
        for &(x, y, _diff) in &differences {
            assert!(x < 10);
            assert!(y < 10);
        }
    }
}