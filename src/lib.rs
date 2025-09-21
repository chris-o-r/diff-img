pub mod blend;
pub mod diff;
pub mod diff_img;
pub mod image_creator;
pub mod lcs;
pub mod perceptual;
pub mod diff_image_error; // Add this line

// Re-export blend functions
pub use blend::{blend_images, BlendMode};

// Re-export the main functions from diff_img module
pub use diff_img::{
    calculate_diff_ratio, highlight_changes_with_color,
};

// Re-export LCS functions
pub use lcs::lcs_diff;

// Re-export perceptual functions
pub use perceptual::{
    create_perceptual_comparison, create_perceptual_diff_image, 
    create_perceptual_diff_only, create_perceptual_heatmap, perceptual_diff
};

// Re-export functions from other modules if needed
pub use diff::{diff, CompareImage};
pub use image_creator::{get_diff_image, mark_org_image};
