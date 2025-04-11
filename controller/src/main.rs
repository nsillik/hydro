//! Blinks an LED
//!
//! This assumes that a LED is connected to GPIO4.
//! Depending on your target and the board you are using you should change the pin.
//! If your board doesn't have on-board LEDs don't forget to add an appropriate resistor.
//!
//!

pub mod display;
pub mod message;
pub mod status;

use std::{sync::mpsc, thread};

use esp_idf_hal::delay::FreeRtos;
// use esp_idf_hal::gpio::*;

fn main() -> anyhow::Result<()> {
    esp_idf_hal::sys::link_patches();
    let (message_tx, message_rx) = mpsc::channel();
    thread::spawn(move || status::main_loop(message_rx));
    thread::spawn(move || display::main_loop(message_tx));

    // let peripherals = Peripherals::take()?;
    // let mut led = PinDriver::output(peripherals.pins.gpio18)?;

    // We set up two threads running our display and status loops
    //
    // The `status` loop will determine what state the hardware (
    // pumps, etc.) should be in, and update our global system status
    // to match. This is the only method that should _write_ our global
    // status.
    //
    // The `display` loop will handle all user i/o. And should be the only
    // _reader_ of our global system status. It however should be able to
    // work with only a copy which is retrieved

    // Keep the watchdog fed
    loop {
        FreeRtos::delay_ms(100);
    }
}

// fn blink<T: OutputPin>(led: &mut PinDriver<'_, T, Output>, delay: u32) {
//     led.set_high().expect("2");
//     FreeRtos::delay_ms(delay);
//     led.set_low().expect("3");
//     FreeRtos::delay_ms(delay);
// }
