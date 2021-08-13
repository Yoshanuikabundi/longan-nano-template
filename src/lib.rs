#![doc = include_str!("../README.md")]

#![no_std]

use embedded_graphics::fonts::{Font12x16, Text};
use embedded_graphics::image::{Image, ImageRaw};
use embedded_graphics::pixelcolor::raw::LittleEndian;
use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::Rectangle;
use embedded_graphics::{primitive_style, text_style};
use longan_nano::lcd::Lcd;

const FERRIS: &[u8] = include_bytes!("ferris.raw");

pub fn draw_ferris(width: i32, height: i32, lcd: &mut Lcd) {
    // Clear screen
    Rectangle::new(Point::new(0, 0), Point::new(width - 1, height - 1))
        .into_styled(primitive_style!(fill_color = Rgb565::BLACK))
        .draw(lcd)
        .unwrap();

    // Load Image Data
    let raw_image: ImageRaw<Rgb565, LittleEndian> = ImageRaw::new(&FERRIS, 86, 64);
    Image::new(&raw_image, Point::new(width / 2 - 43, 0))
        .draw(lcd)
        .unwrap();

    // Set text style
    let style = text_style!(font = Font12x16, text_color = Rgb565::WHITE);

    // Create a text at position (4, 30) and draw it using style defined above
    let message = "I'm Ferris!";
    Text::new(
        message,
        Point::new((width - message.len() as i32 * 12) / 2, height - 16),
    )
    .into_styled(style)
    .draw(lcd)
    .unwrap();
}
