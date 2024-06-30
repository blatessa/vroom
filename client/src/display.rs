use anyhow::Result;
use crossterm::{cursor, execute, terminal, ExecutableCommand};
use std::io::Write;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

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
