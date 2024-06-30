use anyhow::Result;
use image::{DynamicImage, RgbaImage};
use opencv::{
    core, imgproc,
    prelude::*,
    videoio::{VideoCapture, CAP_ANY},
};

pub struct Camera {
    capture: VideoCapture,
}

impl Camera {
    pub fn new() -> Result<Self> {
        let cam = VideoCapture::new(0, CAP_ANY)?;
        if !cam.is_opened()? {
            anyhow::bail!("Unable to open default camera!");
        }
        Ok(Self { capture: cam })
    }

    pub fn capture_frame(&mut self, width: i32, height: i32) -> Result<Option<DynamicImage>> {
        let mut frame = core::Mat::default();
        if !self.capture.read(&mut frame)? || frame.empty() {
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
