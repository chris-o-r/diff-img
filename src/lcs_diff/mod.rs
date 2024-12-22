extern crate base64;
extern crate image;

mod diff;
mod image_creator;

pub use base64::DecodeError;
use diff::*;
use image::*;
use image_creator::*;


pub fn compare(
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
