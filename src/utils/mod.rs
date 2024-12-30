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


pub fn abs_diff(x: u8, y: u8) -> u8 {
    if x > y {
        return x - y;
    }
    return y - x;
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_abs_diff() {
        assert_eq!(abs_diff(5, 8), 3);
        assert_eq!(abs_diff(8, 5), 3);
        assert_eq!(abs_diff(11, 11), 0);
        assert_eq!(abs_diff(0, 255), 255);
    }
}