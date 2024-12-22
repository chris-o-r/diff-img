use image::{DynamicImage, GenericImageView, ImageBuffer, Rgb, RgbImage, Rgba};

pub fn mark_diff_with_color(
    image1: DynamicImage,
    image2: DynamicImage,
    hue: Rgba<u8>,
) -> Result<DynamicImage, String> {
    let mut result: RgbImage = ImageBuffer::new(image1.width(), image2.height());

    image1
        .pixels()
        .into_iter()
        .zip(image2.pixels())
        .map(|(a, b)| if !a.2.eq(&b.2) { (a.0, a.1, hue) } else { a })
        .for_each(|(x, y, pixel)| {
            result.put_pixel(x, y, Rgb([pixel[0], pixel[1], pixel[2]]));
        });

    Ok(DynamicImage::ImageRgb8(result))
}
