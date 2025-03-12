//! Blinks an LED
//!
//! This assumes that a LED is connected to GPIO4.
//! Depending on your target and the board you are using you should change the pin.
//! If your board doesn't have on-board LEDs don't forget to add an appropriate resistor.
//!

use std::thread;

use esp_idf_hal::delay::{Delay, FreeRtos};
use esp_idf_hal::gpio::*;
use esp_idf_hal::peripherals::Peripherals;

fn main() -> anyhow::Result<()> {
    esp_idf_hal::sys::link_patches();

    let peripherals = Peripherals::take()?;
    let mut led = PinDriver::output(peripherals.pins.gpio18)?;
    thread::spawn(move || loop {
        blink(&mut led, 443);
    });

    let mut led = PinDriver::output(peripherals.pins.gpio2)?;
    thread::spawn(move || loop {
        blink(&mut led, 500);
    });

    loop {
        FreeRtos::delay_ms(500);
    }
}

fn blink<T: OutputPin>(led: &mut PinDriver<'_, T, Output>, delay: u32) {
    led.set_high().expect("2");
    FreeRtos::delay_ms(delay);
    led.set_low().expect("3");
    FreeRtos::delay_ms(delay);
}
