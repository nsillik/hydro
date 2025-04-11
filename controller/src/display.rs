use std::sync::mpsc::Sender;

use esp_idf_hal::delay::FreeRtos;

use crate::message::HydroMessage;

pub fn main_loop(_message_tx: Sender<HydroMessage>) {
    loop {
        FreeRtos::delay_ms(100);
    }
}
