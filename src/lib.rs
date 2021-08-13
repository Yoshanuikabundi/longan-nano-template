#![doc = include_str!("../README.md")]

#![no_std]

use embedded_graphics::mono_font::{ascii::FONT_9X18, MonoTextStyle};
use embedded_graphics::text::{Alignment, Baseline, Text, TextStyle, TextStyleBuilder};
use embedded_graphics::image::{Image, ImageRaw};
use embedded_graphics::pixelcolor::raw::LittleEndian;
use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::{Rectangle, PrimitiveStyle};
use longan_nano::lcd::Lcd;

const FERRIS: &[u8] = include_bytes!("ferris.raw");

pub fn draw_ferris(width: i32, height: i32, lcd: &mut Lcd) {
    // Clear screen
    Rectangle::new(Point::new(0, 0), Size::new(width as u32, height as u32))
        .into_styled(PrimitiveStyle::with_fill(Rgb565::BLACK))
        .draw(lcd)
        .unwrap();

    // Load Image Data
    let raw_image: ImageRaw<Rgb565, LittleEndian> = ImageRaw::new(&FERRIS, 86);
    Image::new(&raw_image, Point::new(width as i32 / 2 - 43, 0))
        .draw(lcd)
        .unwrap();

    // Set text style
    let font = &FONT_9X18;
    let char_style = MonoTextStyle::new(font, Rgb565::WHITE);
    let text_style = TextStyleBuilder::new()
        .alignment(Alignment::Center)
        .baseline(Baseline::Bottom)
        .build();

    // Create a text and draw it using style defined above
    let message = "Hey, I'm Ferris!";
    Text::with_text_style(
        message,
        Point::new(width / 2, height),
        char_style,
        text_style
    )
    .draw(lcd)
    .unwrap();
}
