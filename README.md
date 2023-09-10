# Si Crate - Simple Image Manipulation in Rust

The Si crate (pronounced "ess-eye") is a lightweight and easy-to-use Rust library to create Social Share Images (the website previews you see, for example).  

See the examples in `examples` to get started.

> **Note**: The examples require the `image` crate (`si` is based on that) for PNG Encoding. Unless you need to save images in disk (for which you will need to encode the bytes), you don't really need that.

## Installation

To use the Si crate in your Rust project, add it as a dependency in your `Cargo.toml` file:

```toml
[dependencies]
si-img = "0.1"
```

## Getting Started

To start using the Si crate, import the necessary modules:

```rust
use si_img::{SiImage, SiFont};

```

## Creating a Font

You can create a custom font using the `SiFont::new` constructor. You need to provide a font URL and, optionally, font bytes if you have them. Here's an example:

```rust { excludeFromRunAll=true }
let font = SiFont::new("FONT_URL", None); // Second param is font bytes

```

## Creating an Image

Next, create an image using the `SiImage::new` constructor. You'll need to provide an image URL, the font you created earlier, and optionally, image bytes if available:

```rust
let img = SiImage::new("Image_URL", font, None); // Third param is img bytes

```

## Adding Text to the Image

Now, you can add text to the image using the `text` method. Specify the text, scale, x and y coordinates, and an optional color (or use `None` for the default color):

```rust
img.text("Hello Cool User", 48.00, 32.0, 20.0, Some("#00ffff".to_string()));

```

## Getting Image Bytes

Finally, you can retrieve the (decoded) image as bytes using the `as_bytes` method:

```rust
let bytes = img.as_bytes();

```

## Example

Here's a complete example that puts it all together:

```rust
use si_img::{SiImage, SiFont};

fn main() {
    // Create a font
    let font = SiFont::new("FONT_URL", None);

    // Create an image
    let img = SiImage::new("Image_URL", font, None);

    // Add text to the image
    img.text("Hello Cool User", 48.00, 32.0, 20.0, Some("#00ffff".to_string()));

    // Get image bytes
    let bytes = img.as_bytes();

    // Do something with the image bytes (e.g., save to a file or send over a network)
}

```

That's it! You now have the basics to start using the Si crate for simple image manipulation in your Rust projects. Feel free to explore more features and customization options provided by the crate as you build your image processing applications.

> **Note**: This Crate serves as the base for the WebAssembly-based `Si.js` library (for Node.js and Deno).