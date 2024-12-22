use image::{DynamicImage, GenericImageView, Pixel};


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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_calculate_diff_ratio() {
        const EXPECTED_RESULT: f64 = 0.030344018901682257;
        let image1 = image::open("tests/images/image1.png").unwrap();
        let image2 = image::open("tests/images/image2.png").unwrap();
        let result = calculate_diff_ratio(image1, image2);
        assert_eq!(result, EXPECTED_RESULT);
    }
}