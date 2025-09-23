mod blend;
mod diff_image_error;
mod highlight;
mod lcs;
mod numerical;
mod perceptual; // Add this line

// Re-export blend functions
pub use blend::{blend_images, BlendMode};

// Re-export the main functions from diff_img module
pub use highlight::highlight_changes_with_color;

// Re-export LCS functions
pub use lcs::lcs_diff;

// Re-export perceptual functions
pub use perceptual::{
    create_perceptual_diff_image, create_perceptual_diff_only, create_perceptual_heatmap,
    perceptual_diff,
};

// Re-export numerical functions
pub use numerical::calculate_diff_ratio;

pub use diff_image_error::{DiffImgError, Result};