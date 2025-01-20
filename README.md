# DiffImg

**DiffImg** is a command-line tool and libary for comparing two images. It offers multiple modes for image diffing and allows saving the results in different formats. 

## Features
- Calculate the difference ratio between two images.
- Highlight differences with a specific color.
- Perform image comparison using LCS (Longest Common Subsequence).
- Blend two images for visual comparison.
- Save diff results to a file.

## Installation

1. Clone the repository:
   ```bash
   git clone https://github.com/yourusername/diffimg.git
   cd diffimg
   ```

2. Build the project using Cargo:
   ```bash
   cargo build --release
   ```

3. Run the executable:
   ```bash
   ./target/release/diffimg
   ```

## Usage

```bash
diffimg <image1> <image2> [OPTIONS]
```

### Arguments
- `image1` (required): Path to the first image to compare.
- `image2` (required): Path to the second image to compare.

### Options
- `-f, --filename <filename>`: Save the diff image to the specified filename.
- `-m, --mode <mode>`: Set the diff mode. Available modes:
  - `MarkWithColor`
  - `LCS`
  - `Blend`
- `-c, --color <color>`: Specify the color to highlight differences (default: `[0,255,0,0]`).
- `-b, --blend <blend>`: Specify the blend mode. Default is the second blend mode available in `BLEND_MODES`.

### Examples

#### Calculate Diff Ratio
```bash
diffimg image1.png image2.png
```

#### Highlight Differences with a Color
```bash
diffimg image1.png image2.png -m MarkWithColor -c [255,0,0,0] -f output.png
```

#### Blend Two Images
```bash
diffimg image1.png image2.png -m Blend -b Additive -f blended_output.png
```

#### Compare Using LCS
```bash
diffimg image1.png image2.png -m LCS -f lcs_output.png
```

## Configuration
The tool uses predefined modes and blend settings:
- `DIFF_MODES`: A list of supported diff modes.
- `BLEND_MODES`: A list of available blend modes.

### Adding Custom Modes
You can extend `DIFF_MODES` and `BLEND_MODES` in the `config` module to support additional functionality.

## Contributing
Contributions are welcome! Feel free to submit issues or pull requests.

1. Fork the repository.
2. Create a feature branch.
3. Commit your changes.
4. Push to your fork and submit a pull request.

## License
This project is licensed under the MIT License. See the `LICENSE` file for details.

## Acknowledgements
- Built with Rust.
- Image diffing utilities inspired by various open-source libraries.
