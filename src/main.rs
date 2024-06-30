use anyhow::Result;
use crossterm::terminal;
use std::thread;
use std::time::Duration;

use vroom_client::{AsciiRenderer, Camera, Display};

fn main() -> Result<()> {
    let mut camera = Camera::new()?;
    let renderer = AsciiRenderer::default();
    let mut display = Display::new()?;

    loop {
        let (term_width, term_height) = terminal::size()?;
        let img_width = term_width as u32;

        if let Some(frame) = camera.capture_frame(img_width as i32, term_height as i32)? {
            let ascii_frame = renderer.to_ascii(&frame, img_width, term_height as u32);
            display.draw(ascii_frame, img_width)?;
        }

        thread::sleep(Duration::from_millis(100));
    }
}
