#![doc = include_str!("../README.md")]

#![no_std]
#![no_main]

use panic_halt as _;

use embedded_graphics::prelude::*;
use longan_nano::hal::{pac, prelude::*};
use longan_nano::{lcd, lcd_pins};
use riscv_rt::entry;

use {{crate_name}}::draw_ferris;

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();

    // Configure clocks
    let mut rcu = dp
        .RCU
        .configure()
        .ext_hf_clock(8.mhz())
        .sysclk(108.mhz())
        .freeze();
    let mut afio = dp.AFIO.constrain(&mut rcu);

    let gpioa = dp.GPIOA.split(&mut rcu);
    let gpiob = dp.GPIOB.split(&mut rcu);

    let lcd_pins = lcd_pins!(gpioa, gpiob);
    let mut lcd = lcd::configure(dp.SPI0, lcd_pins, &mut afio, &mut rcu);
    let (width, height) = (lcd.size().width as i32, lcd.size().height as i32);

    draw_ferris(width, height, &mut lcd);

    loop {}
}
