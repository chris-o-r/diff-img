use std::fmt;

#[derive(Debug)]
pub enum DiffImgError {
    // Image-related errors
    ImageLoad(String),
    ImageSave(String),
    ImageDimensionMismatch { 
        image1: (u32, u32), 
        image2: (u32, u32) 
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
    
    // Generic errors
    Generic(String),
    
    // Memory/Performance errors
    OutOfMemory,
    ImageTooLarge { width: u32, height: u32, max_size: u64 },
}

impl fmt::Display for DiffImgError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DiffImgError::ImageLoad(path) => 
                write!(f, "Failed to load image: {}", path),
            DiffImgError::ImageSave(path) => 
                write!(f, "Failed to save image: {}", path),
            DiffImgError::ImageDimensionMismatch { image1, image2 } => 
                write!(f, "Image dimensions don't match: {}x{} vs {}x{}", 
                       image1.0, image1.1, image2.0, image2.1),
            DiffImgError::ImageFormat(msg) => 
                write!(f, "Invalid image format: {}", msg),
            DiffImgError::FileNotFound(path) => 
                write!(f, "File not found: {}", path),
            DiffImgError::FileRead(path) => 
                write!(f, "Failed to read file: {}", path),
            DiffImgError::FileWrite(path) => 
                write!(f, "Failed to write file: {}", path),
            DiffImgError::Base64Decode(err) => 
                write!(f, "Base64 decode error: {}", err),
            DiffImgError::InvalidAlgorithm(algo) => 
                write!(f, "Invalid algorithm: {}", algo),
            DiffImgError::AlgorithmFailed(msg) => 
                write!(f, "Algorithm failed: {}", msg),
            DiffImgError::InvalidBlendMode(mode) => 
                write!(f, "Invalid blend mode: {}", mode),
            DiffImgError::InvalidColor(color) => 
                write!(f, "Invalid color specification: {}", color),
            DiffImgError::InvalidThreshold(threshold) => 
                write!(f, "Invalid threshold value: {} (must be between 0.0 and 1.0)", threshold),
            DiffImgError::Generic(msg) => 
                write!(f, "Error: {}", msg),
            DiffImgError::OutOfMemory => 
                write!(f, "Out of memory"),
            DiffImgError::ImageTooLarge { width, height, max_size } => 
                write!(f, "Image too large: {}x{} pixels (max allowed: {} pixels)", 
                       width, height, max_size),
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

// Helper functions for common validations
impl DiffImgError {
    pub fn check_dimensions(img1: (u32, u32), img2: (u32, u32)) -> Result<()> {
        if img1 != img2 {
            Err(DiffImgError::ImageDimensionMismatch { 
                image1: img1, 
                image2: img2 
            })
        } else {
            Ok(())
        }
    }
    
    pub fn check_threshold(threshold: f32) -> Result<()> {
        if threshold < 0.0 || threshold > 1.0 {
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
                max_size: max_pixels 
            })
        } else {
            Ok(())
        }
    }
}