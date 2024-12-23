use base64::encode;
use image::Rgba;

#[derive(Debug)]
pub struct CompareImage {
    dimensions: (u32, u32),
    pixels: Vec<Rgba<u8>>,
}

impl CompareImage {
    pub fn new(dimensions: (u32, u32), pixels: Vec<Rgba<u8>>) -> Self {
        CompareImage { dimensions, pixels }
    }

    pub fn create_encoded_rows(&self) -> Vec<String> {
        let mut rows = Vec::new();
        let mut row = Vec::new();
        for pixel in &self.pixels {
            row.push(pixel.0[0]);
            row.push(pixel.0[1]);
            row.push(pixel.0[2]);
            row.push(pixel.0[3]);
            if row.len() == self.dimensions.0 as usize * 4 {
                rows.push(encode(&row));
                row.clear();
            }
        }
        rows
    }
}

pub fn diff(imga: CompareImage, imgb: CompareImage) -> Vec<lcs_diff::DiffResult<String>> {
    let imga = imga.create_encoded_rows();
    let imgb = imgb.create_encoded_rows();
    lcs_diff::diff(&imga, &imgb)
}
