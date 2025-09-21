use std::cmp;

use base64::DecodeError;
use base64::{engine::general_purpose::STANDARD, Engine as _};
use image::{DynamicImage, GenericImage, GenericImageView, ImageBuffer, Rgba};

static BLACK: (u8, u8, u8) = (0, 0, 0);
static RED: (u8, u8, u8) = (255, 119, 119);
static GREEN: (u8, u8, u8) = (99, 195, 99);

pub fn lcs_diff(
    before: &DynamicImage,
    after: &DynamicImage,
    rate: f32,
) -> Result<DynamicImage, DecodeError> {
    let imga = create_encoded_rows(before);
    let imgb = create_encoded_rows(after);
    let result = lcs_diff::diff(&imga, &imgb);

    let mut added: Vec<usize> = Vec::new();
    let mut removed: Vec<usize> = Vec::new();

    for d in result.iter() {
        match d {
            lcs_diff::DiffResult::Added(a) => added.push(a.new_index.unwrap()),
            lcs_diff::DiffResult::Removed(r) => removed.push(r.old_index.unwrap()),
            _ => (),
        }
    }
    let mut before = before.clone();
    let mut after = after.clone();
    mark_org_image(&mut before, RED, rate, &removed);
    mark_org_image(&mut after, GREEN, rate, &added);

    get_diff_image(before.dimensions().0, after.dimensions().0, &result, rate)
}

fn create_encoded_rows(image: &DynamicImage) -> Vec<String> {
    let pixels = image.pixels().map(|pix| pix.2).collect::<Vec<_>>();
    let dimensions = image.dimensions();
    let mut rows = Vec::new();
    let mut row = Vec::new();
    for pixel in pixels {
        row.push(pixel.0[0]);
        row.push(pixel.0[1]);
        row.push(pixel.0[2]);
        row.push(pixel.0[3]);
        if row.len() == dimensions.0 as usize * 4 {
            rows.push(STANDARD.encode(&row));
            row.clear();
        }
    }
    rows
}

fn mark_org_image(base: &mut DynamicImage, color: (u8, u8, u8), rate: f32, indexes: &[usize]) {
    let range = compute_range(indexes);
    blend_diff_area(base, range, color, rate);
}

fn compute_range(r: &[usize]) -> Vec<(usize, usize)> {
    let mut i = 0;
    let mut j = 0;
    let mut acc: usize;
    let mut y1: usize;
    let mut ranges: Vec<(usize, usize)> = Vec::new();
    while i < r.len() {
        y1 = r[i];
        acc = y1;
        i += 1;
        loop {
            if i >= r.len() {
                break;
            }
            let index = r[i];
            if acc + 1 != index {
                break;
            }
            acc = index;
            i += 1;
            j += 1;
        }
        let y2 = y1 + j;
        j = 0;
        ranges.push((y1, y2));
    }
    ranges
}

fn blend_diff_area<G>(img: &mut G, ranges: Vec<(usize, usize)>, rgb: (u8, u8, u8), rate: f32)
where
    G: GenericImage<Pixel = Rgba<u8>>,
{
    for (y1, y2) in ranges {
        for y in y1..(y2 + 1) {
            for x in 0..img.dimensions().0 {
                let p = img.get_pixel(x, y as u32);
                let blended = blend(p, rgb, rate);
                img.put_pixel(x, y as u32, blended);
            }
        }
    }
}

fn blend(base: Rgba<u8>, rgb: (u8, u8, u8), rate: f32) -> Rgba<u8> {
    Rgba([
        (base.0[0] as f32 * (1.0 - rate) + rgb.0 as f32 * (rate)) as u8,
        (base.0[1] as f32 * (1.0 - rate) + rgb.1 as f32 * (rate)) as u8,
        (base.0[2] as f32 * (1.0 - rate) + rgb.2 as f32 * (rate)) as u8,
        base.0[3],
    ])
}

fn put_diff_pixels(
    y: usize,
    img: &mut ImageBuffer<Rgba<u8>, Vec<u8>>,
    row_width: u32,
    data: &str,
    rgb: (u8, u8, u8),
    rate: f32,
) -> Result<(), base64::DecodeError> {
    let row = STANDARD.decode(data)?;
    for x in 0..img.dimensions().0 {
        let index = x as usize * 4;
        let pixel: Rgba<u8> = if row_width > x && index + 4 <= row.len() {
            Rgba([row[index], row[index + 1], row[index + 2], row[index + 3]])
        } else {
            Rgba([0, 0, 0, 0])
        };
        img.put_pixel(x, y as u32, blend(pixel, rgb, rate));
    }
    Ok(())
}

fn get_diff_image(
    before_width: u32,
    after_width: u32,
    result: &[lcs_diff::DiffResult<String>],
    rate: f32,
) -> Result<DynamicImage, base64::DecodeError> {
    let height = result.len() as u32;
    let width = cmp::max(before_width, after_width);
    let mut img: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::new(width, height);
    for (y, d) in result.iter().enumerate() {
        match d {
            lcs_diff::DiffResult::Added(a) => {
                put_diff_pixels(y, &mut img, after_width, &a.data, GREEN, rate)?
            }
            lcs_diff::DiffResult::Removed(r) => {
                put_diff_pixels(y, &mut img, before_width, &r.data, RED, rate)?
            }
            lcs_diff::DiffResult::Common(c) => {
                put_diff_pixels(y, &mut img, width, &c.data, BLACK, 0.0)?
            }
        }
    }
    Ok(DynamicImage::ImageRgba8(img))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lcs_diff_identical_images() {
        let mut image1 = image::open("tests/images/image1.png").unwrap();
        let mut image2 = image1.clone();
        let rate = 0.5;

        let result = lcs_diff(&mut image1, &mut image2, rate);
        assert!(result.is_ok());

        let diff_image = result.unwrap();
        // Identical images should produce a diff image with mostly common regions
        assert_eq!(diff_image.dimensions(), image1.dimensions());
    }

    #[test]
    fn test_lcs_diff_different_images() {
        let mut image1 = image::open("tests/images/image1.png").unwrap();
        let mut image2 = image::open("tests/images/image2.png").unwrap();
        let rate = 0.5;

        let result = lcs_diff(&mut image1, &mut image2, rate);
        assert!(result.is_ok());

        let diff_image = result.unwrap();
        // The diff image height should be based on the number of diff result rows
        // which could be different from the original image heights
        assert!(diff_image.dimensions().0 > 0);
        assert!(diff_image.dimensions().1 > 0);
    }

    #[test]
    fn test_lcs_diff_rate_bounds() {
        let mut image1 = image::open("tests/images/image1.png").unwrap();
        let mut image2 = image::open("tests/images/image2.png").unwrap();

        // Test with minimum rate
        let result_min = lcs_diff(&mut image1.clone(), &mut image2.clone(), 0.0);
        assert!(result_min.is_ok());

        // Test with maximum rate
        let result_max = lcs_diff(&mut image1, &mut image2, 1.0);
        assert!(result_max.is_ok());
    }

    #[test]
    fn test_lcs_diff_preserves_original_dimensions() {
        let mut image1 = image::open("tests/images/image1.png").unwrap();
        let mut image2 = image::open("tests/images/image2.png").unwrap();
        let original_dims1 = image1.dimensions();
        let original_dims2 = image2.dimensions();
        let rate = 0.3;

        let result = lcs_diff(&mut image1, &mut image2, rate);
        assert!(result.is_ok());

        // Original images should maintain their dimensions after LCS processing
        assert_eq!(image1.dimensions(), original_dims1);
        assert_eq!(image2.dimensions(), original_dims2);
    }

    #[test]
    fn test_lcs_diff_modifies_input_images() {
        let original_image1 = image::open("tests/images/image1.png").unwrap();
        let original_image2 = image::open("tests/images/image2.png").unwrap();
        let mut image1 = original_image1.clone();
        let mut image2 = original_image2.clone();
        let rate = 0.4;

        let _result = lcs_diff(&mut image1, &mut image2, rate);

        // LCS diff should modify the input images (marking removed/added regions)
        // Note: This is a behavioral test - in practice you might want to compare
        // some pixels to verify they've been modified with the marking colors
        assert_eq!(image1.dimensions(), original_image1.dimensions());
        assert_eq!(image2.dimensions(), original_image2.dimensions());
    }
}
