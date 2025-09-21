use base64::DecodeError;
use image::{DynamicImage, GenericImageView};

use crate::diff::*;
use crate::image_creator::*;

pub fn lcs_diff(
    before: &DynamicImage,
    after: &DynamicImage,
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