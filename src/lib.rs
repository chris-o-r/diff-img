pub mod diff;
pub mod diff_img;
pub mod image_creator;

// Re-export the main functions from diff_img module
pub use diff_img::{
    blend_images, calculate_diff_ratio, highlight_changes_with_color, lcs_diff, BlendMode,
};

// Re-export functions from other modules if needed
pub use diff::{diff, CompareImage};
pub use image_creator::{get_diff_image, mark_org_image};
