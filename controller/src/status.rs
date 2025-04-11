use std::sync::mpsc::Receiver;

use esp_idf_hal::delay::FreeRtos;

use crate::message::HydroMessage;

pub fn main_loop(_message_rx: Receiver<HydroMessage>) {
    loop {
        FreeRtos::delay_ms(100);
    }
}
