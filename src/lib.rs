use anyhow::Result;
use crossterm::{cursor, execute, terminal, ExecutableCommand};
use image::{DynamicImage, GenericImageView, RgbaImage};
use opencv::{
    core, imgproc,
    prelude::*,
    videoio::{VideoCapture, CAP_ANY},
};
use std::io::Write;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

pub struct AsciiRenderer {
    ascii_chars: Vec<char>,
}

impl AsciiRenderer {
    pub fn new() -> Self {
        let ascii_chars = "$@B%8&WM#*oahkbdpqwmZO0QLCJUYXzcvunxrjft"
            .chars()
            .rev()
            .collect::<Vec<_>>();
        Self { ascii_chars }
    }

    pub fn to_ascii(&self, image: &DynamicImage, width: u32, height: u32) -> Vec<(char, Color)> {
        let mut ascii_frame = Vec::with_capacity((width * height) as usize);

        for y in 0..height {
            for x in 0..width {
                let pixel = image.get_pixel(x, y);
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

pub struct Camera {
    cam: VideoCapture,
}

impl Camera {
    pub fn new() -> Result<Self> {
        let cam = VideoCapture::new(0, CAP_ANY)?;
        if !cam.is_opened()? {
            anyhow::bail!("Unable to open default camera!");
        }
        Ok(Self { cam })
    }

    pub fn capture_frame(&mut self, width: i32, height: i32) -> Result<Option<DynamicImage>> {
        let mut frame = core::Mat::default();
        if !self.cam.read(&mut frame)? || frame.empty() {
            return Ok(None);
        }

        let mut resized_frame = core::Mat::default();
        imgproc::resize(
            &frame,
            &mut resized_frame,
            core::Size::new(width, height),
            0.0,
            0.0,
            imgproc::INTER_CUBIC,
        )?;
        let mut rgba_frame = core::Mat::default();
        imgproc::cvt_color(&resized_frame, &mut rgba_frame, imgproc::COLOR_BGR2RGBA, 0)?;

        let data = rgba_frame.data_bytes()?;
        if let Some(rgba_image) = RgbaImage::from_raw(width as u32, height as u32, data.to_vec()) {
            Ok(Some(DynamicImage::ImageRgba8(rgba_image)))
        } else {
            Ok(None)
        }
    }
}

pub struct Display {
    stdout: StandardStream,
    previous_frame: Vec<(char, Color)>,
}

impl Display {
    pub fn new() -> Result<Self> {
        let mut stdout = StandardStream::stdout(ColorChoice::Always);
        stdout.execute(terminal::EnterAlternateScreen)?;
        stdout.execute(terminal::Clear(terminal::ClearType::All))?;
        Ok(Self {
            stdout,
            previous_frame: Vec::new(),
        })
    }

    pub fn draw(&mut self, ascii_frame: Vec<(char, Color)>, img_width: u32) -> Result<()> {
        if self.previous_frame.is_empty() {
            self.previous_frame = vec![(' ', Color::Black); ascii_frame.len()];
        }

        for (i, (char, color)) in ascii_frame.iter().enumerate() {
            if i >= self.previous_frame.len() {
                break;
            }
            let (prev_char, prev_color) = &self.previous_frame[i];
            if char != prev_char || color != prev_color {
                let x = (i as u32 % img_width) as u16;
                let y = (i as u32 / img_width) as u16;
                execute!(self.stdout, cursor::MoveTo(x, y))?;
                self.stdout
                    .set_color(ColorSpec::new().set_fg(Some(*color)))?;
                write!(self.stdout, "{}", char)?;
                self.previous_frame[i] = (*char, *color);
            }
        }
        self.stdout.reset()?;
        self.stdout.flush()?;
        Ok(())
    }
}
