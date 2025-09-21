use image::DynamicImage;

pub fn safe_save_image(image: DynamicImage, filename: &str) -> Result<String, String> {
    // Check if path exists
    let path = std::path::Path::new(filename);

    if !path.parent().unwrap().exists() {
        return Err(format!(
            "Path {} does not exist",
            path.parent().unwrap().display()
        ));
    }

    if let Err(msg) = image.save(filename) {
        return Err(msg.to_string());
    }

    Ok(filename.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_safe_save_image_invalid_path() {
        let img = DynamicImage::new_rgb8(1, 1);
        let result = safe_save_image(img, "/invalid/path/to/file.png");
        assert!(result.is_err());
    }
}
