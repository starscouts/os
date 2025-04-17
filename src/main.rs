use embedded_graphics_framebuffer::FrameBufferDisplay;
use std::time::Duration;
use crossterm::cursor::{Hide, Show};
use crossterm::event::{poll, read, Event, KeyCode, KeyEvent, KeyModifiers};
use embedded_graphics::{
    pixelcolor::{Rgb888, RgbColor},
    prelude::*,
};
use embedded_graphics::mono_font::ascii::{FONT_10X20, FONT_4X6, FONT_9X18_BOLD};
use embedded_graphics::mono_font::iso_8859_10::FONT_9X18;
use embedded_graphics::mono_font::iso_8859_13::{FONT_5X8, FONT_6X10};
use embedded_graphics::mono_font::iso_8859_7::FONT_9X15;
use embedded_graphics::mono_font::MonoTextStyle;
use embedded_graphics::text::Text;
use framebuffer::{Framebuffer, KdMode};

const FRAME_RATE: usize = 5;
const MS_PER_FRAME: usize = 1000 / FRAME_RATE;

fn main() {
    let mut display = FrameBufferDisplay::new();
    print!("{}", Hide);
    Framebuffer::set_kd_mode(KdMode::Graphics).unwrap();
    crossterm::terminal::enable_raw_mode().expect("Failed to configure keyboard");

    let mut text = format!("{name} {version}\nBuild {date}\n{path}\n{host} -> {target} ({profile}, {compiler})\n\nOS> ",
        name = env!("CARGO_PKG_NAME"), version = env!("CARGO_PKG_VERSION"),
        date = env!("DATE"), path = env!("OUT_DIR"),
        host = env!("CARGO_HOST"), target = env!("CARGO_TARGET"),
        profile = env!("CARGO_PROFILE"), compiler = env!("CARGO_RUSTC")
    );

    'main: loop {
        display.clear(Rgb888::BLUE).expect("Failed to clear the screen");

        Text::new(&format!("{}_", text), Point::new(0, 10), MonoTextStyle::new(&FONT_9X18_BOLD, Rgb888::WHITE))
            .draw(&mut display)
            .expect("Failed to draw");

        display.flush().expect("Failed to push image to screen");

        if poll(Duration::from_millis(MS_PER_FRAME as u64)).unwrap() {
            match read().unwrap() {
                Event::Key(event) => {
                    match event {
                        KeyEvent {
                            code: KeyCode::Char('c'),
                            modifiers: KeyModifiers::CONTROL,
                            ..
                        } => {
                            print!("{}", Show);
                            Framebuffer::set_kd_mode(KdMode::Text).unwrap();
                            crossterm::terminal::disable_raw_mode().expect("Failed to unconfigure keyboard");
                            break 'main;
                        },
                        _ => {
                            match event.code {
                                KeyCode::Char(char) => {
                                    text.push(char)
                                },
                                KeyCode::Enter => {
                                    text.push('\n');
                                },
                                KeyCode::Backspace => {
                                    text = text.strip_suffix(|_: char| true).unwrap_or("").to_string()
                                }
                                _ => {}
                            }
                        }
                    }
                }
                _ => {}
            }
        }
    }
}