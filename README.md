# diff_img

**diff_img** is a powerful command-line tool and Rust library for comparing and analyzing differences between images. It provides multiple algorithms and visualization modes for image comparison, making it useful for testing, quality assurance, and visual analysis.

## Features

- **Multiple Diff Algorithms**: Choose from different algorithms optimized for various use cases
  - **Solid Color**: Highlight differences with a customizable color
  - **LCS (Longest Common Subsequence)**: Advanced diff algorithm for detailed comparison
  - **Blend**: Combine images using various blend modes for visual analysis
  - **Perceptual**: Human vision-inspired comparison using Delta E color space analysis
- **Flexible Output**: Calculate difference ratios or save visual diff images
- **Customizable Visualization**: Multiple blend modes and color options
- **Library Support**: Use as a Rust crate in your own projects
- **Comprehensive Error Handling**: Robust error reporting and handling

## Installation

### From Source

1. Clone the repository:
   ```bash
   git clone https://github.com/chris-o-r/diff-img.git
   cd diff-img
   ```

2. Build the project using Cargo:
   ```bash
   cargo build --release
   ```

3. The executable will be available at:
   ```bash
   ./target/release/diff_img
   ```

### As a Library

Add this to your `Cargo.toml`:
```toml
[dependencies]
diff_img = "0.1.0"
```

## Usage

### Command Line Interface

```bash
diff_img <image1> <image2> [OPTIONS]
```

### Arguments
- `<image1>` (required): Path to the first image to compare
- `<image2>` (required): Path to the second image to compare

### Options
- `-m, --mode <mode>`: Diff algorithm to use. Available modes:
  - `solid-color`: Highlight differences with a solid color
  - `lcs`: Longest Common Subsequence algorithm
  - `blend`: Blend images using specified blend mode
  - `perceptual`: Perceptual color difference analysis using Delta E
- `-c, --color <color>`: Color for highlighting differences (default: `[0,255,0,0]` - green)
- `-b, --blend <blend>`: Blend mode when using blend diff (default: `hue`):
  - `bias`: Bias-based blending
  - `hue`: Hue-based color blending  
  - `overlay`: Overlay blending
- `-f, --filename <filename>`: Save diff image to specified file (required when using diff modes)
- `-h, --help`: Show help information
- `-V, --version`: Show version information

### Examples

#### Calculate Basic Diff Ratio
```bash
diff_img image1.png image2.png
```
Output: `Diff ratio 0.030344018901682257`

#### Highlight Differences with Solid Color
```bash
diff_img image1.png image2.png -m solid-color -c [255,0,0,255] -f differences.png
```

#### Advanced LCS Comparison
```bash
diff_img image1.png image2.png -m lcs -f lcs_diff.png
```

#### Blend Mode Analysis
```bash
diff_img image1.png image2.png -m blend -b overlay -f blended.png
```

#### Perceptual Difference Analysis
```bash
diff_img image1.png image2.png -m perceptual -f perceptual_diff.png
```

## Library Usage

The `diff_img` crate provides a comprehensive API for image comparison:

```rust
use diff_img::{
    blend::{blend_images, BlendMode},
    perceptual::create_perceptual_diff_image,
    lcs::lcs_diff,
    diff_img::calculate_diff_ratio,
};
use image::DynamicImage;

// Calculate difference ratio
let image1 = image::open("image1.png")?;
let image2 = image::open("image2.png")?;
let ratio = calculate_diff_ratio(image1.clone(), image2.clone());

// Create perceptual diff
let diff_img = create_perceptual_diff_image(image1.clone(), image2.clone(), 2.0)?;

// Blend images
let blended = blend_images(image1, image2, BlendMode::Overlay)?;
```

## Algorithms

### Solid Color Mode
Highlights pixel differences using a specified color, ideal for simple before/after comparisons.

### LCS (Longest Common Subsequence)
Uses dynamic programming to find the longest sequence of matching pixels, providing detailed change analysis.

### Blend Mode
Combines images using various blend algorithms:
- **Bias**: Applies color bias based on pixel differences
- **Hue**: Creates purple-tinted highlights for differences  
- **Overlay**: Standard overlay blending for visual comparison

### Perceptual Mode
Uses Delta E color difference calculation in LAB color space to match human visual perception, with configurable sensitivity thresholds.

## Project Structure

The codebase is organized into focused modules:

```
src/
├── lib.rs              # Library entry point and module exports
├── main.rs             # CLI application entry point
├── config.rs           # Configuration and CLI argument parsing
├── diff_img.rs         # Core image difference functionality
├── blend.rs            # Image blending algorithms
├── lcs.rs              # Longest Common Subsequence implementation
├── perceptual.rs       # Perceptual color difference analysis
├── image_creator.rs    # Image creation and manipulation utilities
├── diff.rs             # Core diff structures and traits
├── diff_image_error.rs # Comprehensive error handling
└── utils/              # Utility functions and helpers
```

## Testing

The project includes comprehensive test coverage:

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific module tests
cargo test blend::tests
cargo test perceptual::tests
```

Current test coverage includes 45+ tests across all modules, ensuring reliability and correctness.

## Performance

- **Optimized algorithms**: Efficient implementations for large image processing
- **Memory safety**: Rust's ownership system prevents common image processing errors
- **Parallel processing**: Some operations utilize Rust's excellent parallel processing capabilities
- **Error handling**: Comprehensive error handling prevents crashes and provides clear feedback

## Contributing

Contributions are welcome! Please follow these steps:

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Make your changes with tests
4. Ensure all tests pass (`cargo test`)
5. Commit your changes (`git commit -m 'Add amazing feature'`)
6. Push to the branch (`git push origin feature/amazing-feature`)
7. Open a Pull Request

### Development Guidelines

- Write tests for new functionality
- Follow Rust naming conventions
- Document public APIs
- Ensure `cargo clippy` passes without warnings
- Format code with `cargo fmt`

## Dependencies

- **clap** (4.5.40): Command-line argument parsing
- **image** (0.25.6): Image processing and format support
- **base64** (0.22.1): Base64 encoding support
- **lcs-diff** (0.1.1): LCS algorithm implementation

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Acknowledgements

- Built with [Rust](https://www.rust-lang.org/) for performance and safety
- Image processing powered by the [image](https://crates.io/crates/image) crate
- CLI interface built with [clap](https://crates.io/crates/clap)
- LCS algorithm implementation from [lcs-diff](https://crates.io/crates/lcs-diff)

## Changelog

### Version 0.1.0
- Initial release with four diff algorithms
- Comprehensive CLI interface
- Library API for programmatic use
- Extensive test coverage
- Modular architecture for easy extension
