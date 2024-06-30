use image::{imageops::FilterType, DynamicImage, GenericImageView};
use termcolor::Color;

pub struct AsciiRenderer {
    ascii_chars: Vec<char>,
}

impl AsciiRenderer {
    pub fn new() -> Self {
        // dark to lights
        let ascii_chars = "$@B%8&WM#*oahkbdpqwmZO0QLCJUYXzcvunxrjft"
            .chars()
            .rev()
            .collect::<Vec<_>>();
        Self { ascii_chars }
    }

    fn adjust_brightness_contrast(
        image: &DynamicImage,
        brightness: f32,
        contrast: f32,
    ) -> DynamicImage {
        let mut adjusted = image.clone();
        adjusted = adjusted.brighten((brightness * 255.0) as i32);
        adjusted = adjusted.adjust_contrast(contrast);
        adjusted
    }

    pub fn to_ascii(&self, image: &DynamicImage, width: u32, height: u32) -> Vec<(char, Color)> {
        let resized_image = image.resize_exact(width, height, FilterType::Lanczos3);

        // Adjust brightness and contrast based on calculated differences
        let preprocessed_image = Self::adjust_brightness_contrast(&resized_image, 0.0, 0.8); // Fine-tuning values

        let mut ascii_frame = Vec::with_capacity((width * height) as usize);

        for y in 0..height {
            for x in 0..width {
                let pixel = preprocessed_image.get_pixel(x, y);
                let gray =
                    (0.299 * pixel[0] as f32 + 0.587 * pixel[1] as f32 + 0.114 * pixel[2] as f32)
                        / 255.0;
                let index = (gray * (self.ascii_chars.len() - 1) as f32).floor() as usize;
                let color = Color::Rgb(pixel[0], pixel[1], pixel[2]);
                ascii_frame.push((self.ascii_chars[index], color));
            }
        }

        ascii_frame
    }
}

impl Default for AsciiRenderer {
    fn default() -> Self {
        Self::new()
    }
}
