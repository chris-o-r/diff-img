use std::fmt;

#[derive(Debug)]
pub enum DiffImgError {
    // Image-related errors
    ImageLoad(String),
    ImageSave(String),
    ImageDimensionMismatch {
        image1: (u32, u32),
        image2: (u32, u32),
    },
    ImageFormat(String),

    // File I/O errors
    FileNotFound(String),
    FileRead(String),
    FileWrite(String),

    // Encoding/Decoding errors
    Base64Decode(base64::DecodeError),

    // Algorithm-specific errors
    InvalidAlgorithm(String),
    AlgorithmFailed(String),

    // Configuration errors
    InvalidBlendMode(String),
    InvalidColor(String),
    InvalidThreshold(f32),
    InvalidDiffMode(String),

    // Generic errors
    Generic(String),

    // Memory/Performance errors
    OutOfMemory,
    ImageTooLarge {
        width: u32,
        height: u32,
        max_size: u64,
    },
}

impl fmt::Display for DiffImgError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DiffImgError::ImageLoad(path) => write!(f, "Failed to load image: {}", path),
            DiffImgError::ImageSave(path) => write!(f, "Failed to save image: {}", path),
            DiffImgError::ImageDimensionMismatch { image1, image2 } => write!(
                f,
                "Image dimensions don't match: {}x{} vs {}x{}",
                image1.0, image1.1, image2.0, image2.1
            ),
            DiffImgError::ImageFormat(msg) => write!(f, "Invalid image format: {}", msg),
            DiffImgError::FileNotFound(path) => write!(f, "File not found: {}", path),
            DiffImgError::FileRead(path) => write!(f, "Failed to read file: {}", path),
            DiffImgError::FileWrite(path) => write!(f, "Failed to write file: {}", path),
            DiffImgError::Base64Decode(err) => write!(f, "Base64 decode error: {}", err),
            DiffImgError::InvalidAlgorithm(algo) => write!(f, "Invalid algorithm: {}", algo),
            DiffImgError::AlgorithmFailed(msg) => write!(f, "Algorithm failed: {}", msg),
            DiffImgError::InvalidBlendMode(mode) => write!(f, "Invalid blend mode: {}", mode),
            DiffImgError::InvalidDiffMode(mode) => write!(f, "Invalid diff mode: {}", mode),
            DiffImgError::InvalidColor(color) => {
                write!(f, "Invalid color specification: {}", color)
            }
            DiffImgError::InvalidThreshold(threshold) => write!(
                f,
                "Invalid threshold value: {} (must be between 0.0 and 1.0)",
                threshold
            ),
            DiffImgError::Generic(msg) => write!(f, "Error: {}", msg),
            DiffImgError::OutOfMemory => write!(f, "Out of memory"),
            DiffImgError::ImageTooLarge {
                width,
                height,
                max_size,
            } => write!(
                f,
                "Image too large: {}x{} pixels (max allowed: {} pixels)",
                width, height, max_size
            ),
        }
    }
}

impl std::error::Error for DiffImgError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            DiffImgError::Base64Decode(err) => Some(err),
            _ => None,
        }
    }
}

// Convenience type alias
pub type Result<T> = std::result::Result<T, DiffImgError>;

// From implementations for easy conversion
impl From<image::ImageError> for DiffImgError {
    fn from(err: image::ImageError) -> Self {
        DiffImgError::ImageLoad(err.to_string())
    }
}

impl From<std::io::Error> for DiffImgError {
    fn from(err: std::io::Error) -> Self {
        match err.kind() {
            std::io::ErrorKind::NotFound => DiffImgError::FileNotFound(err.to_string()),
            std::io::ErrorKind::PermissionDenied => DiffImgError::FileRead(err.to_string()),
            _ => DiffImgError::Generic(err.to_string()),
        }
    }
}

impl From<base64::DecodeError> for DiffImgError {
    fn from(err: base64::DecodeError) -> Self {
        DiffImgError::Base64Decode(err)
    }
}

impl From<String> for DiffImgError {
    fn from(msg: String) -> Self {
        DiffImgError::Generic(msg)
    }
}

impl From<&str> for DiffImgError {
    fn from(msg: &str) -> Self {
        DiffImgError::Generic(msg.to_string())
    }
}

impl PartialEq for DiffImgError {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (DiffImgError::ImageLoad(a), DiffImgError::ImageLoad(b)) => a == b,
            (DiffImgError::ImageSave(a), DiffImgError::ImageSave(b)) => a == b,
            (
                DiffImgError::ImageDimensionMismatch {
                    image1: a1,
                    image2: a2,
                },
                DiffImgError::ImageDimensionMismatch {
                    image1: b1,
                    image2: b2,
                },
            ) => a1 == b1 && a2 == b2,
            (DiffImgError::ImageFormat(a), DiffImgError::ImageFormat(b)) => a == b,
            (DiffImgError::FileNotFound(a), DiffImgError::FileNotFound(b)) => a == b,
            (DiffImgError::FileRead(a), DiffImgError::FileRead(b)) => a == b,
            (DiffImgError::FileWrite(a), DiffImgError::FileWrite(b)) => a == b,
            (DiffImgError::Base64Decode(_), DiffImgError::Base64Decode(_)) => true, // Can't compare inner error
            (DiffImgError::InvalidAlgorithm(a), DiffImgError::InvalidAlgorithm(b)) => a == b,
            (DiffImgError::AlgorithmFailed(a), DiffImgError::AlgorithmFailed(b)) => a == b,
            (DiffImgError::InvalidBlendMode(a), DiffImgError::InvalidBlendMode(b)) => a == b,
            (DiffImgError::InvalidDiffMode(a), DiffImgError::InvalidDiffMode(b)) => a == b,
            (DiffImgError::InvalidColor(a), DiffImgError::InvalidColor(b)) => a == b,
            (DiffImgError::InvalidThreshold(a), DiffImgError::InvalidThreshold(b)) => a == b,
            (DiffImgError::Generic(a), DiffImgError::Generic(b)) => a == b,
            (DiffImgError::OutOfMemory, DiffImgError::OutOfMemory) => true,
            (
                DiffImgError::ImageTooLarge {
                    width: aw,
                    height: ah,
                    max_size: am,
                },
                DiffImgError::ImageTooLarge {
                    width: bw,
                    height: bh,
                    max_size: bm,
                },
            ) => aw == bw && ah == bh && am == bm,
            _ => false,
        }
    }
}

// Helper functions for common validations
impl DiffImgError {
    pub fn check_dimensions(img1: (u32, u32), img2: (u32, u32)) -> Result<()> {
        if img1 != img2 {
            Err(DiffImgError::ImageDimensionMismatch {
                image1: img1,
                image2: img2,
            })
        } else {
            Ok(())
        }
    }

    pub fn check_threshold(threshold: f32) -> Result<()> {
        if !(0.0..=1.0).contains(&threshold) {
            Err(DiffImgError::InvalidThreshold(threshold))
        } else {
            Ok(())
        }
    }

    pub fn check_image_size(width: u32, height: u32, max_pixels: u64) -> Result<()> {
        let total_pixels = width as u64 * height as u64;
        if total_pixels > max_pixels {
            Err(DiffImgError::ImageTooLarge {
                width,
                height,
                max_size: max_pixels,
            })
        } else {
            Ok(())
        }
    }
}
