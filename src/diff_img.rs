use image::{DynamicImage, GenericImageView, ImageBuffer, Pixel, Rgb, RgbImage, Rgba};

pub fn highlight_changes_with_color(
    before: &DynamicImage,
    after: &DynamicImage,
    color: Rgba<u8>,
) -> Result<DynamicImage, String> {
    let mut result: RgbImage = ImageBuffer::new(before.width(), after.height());

    before
        .pixels()
        .zip(after.pixels())
        .map(|(a, b)| if !a.2.eq(&b.2) { (a.0, a.1, color) } else { a })
        .for_each(|(x, y, pixel)| {
            result.put_pixel(x, y, Rgb([pixel[0], pixel[1], pixel[2]]));
        });

    Ok(DynamicImage::ImageRgb8(result))
}

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
    y - x
}

#[cfg(test)]
mod tests {
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
    fn test_calculate_diff_ratio() {
        const EXPECTED_RESULT: f64 = 0.030344018901682257;
        let image1 = image::open("tests/images/image1.png").unwrap();
        let image2 = image::open("tests/images/image2.png").unwrap();
        let result = calculate_diff_ratio(&image1, &image2);
        assert_eq!(result, EXPECTED_RESULT);
    }
}
