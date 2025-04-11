#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use esp_backtrace as _;
use esp_hal::gpio::{Input, Output};
use log::info;

extern crate alloc;

#[esp_hal_embassy::main]
async fn main(spawner: Spawner) {
    // generator version: 0.3.1

    esp_println::logger::init_logger_from_env();

    let config = esp_hal::Config::default();
    let peripherals = esp_hal::init(config);

    // Initialize heap (this is done by esp_alloc macro)
    esp_alloc::heap_allocator!(size: 72 * 1024);

    let timer0 = esp_hal::timer::systimer::SystemTimer::new(peripherals.SYSTIMER);
    esp_hal_embassy::init(timer0.alarm0);

    info!("Embassy initialized!");

    let timer1 = esp_hal::timer::timg::TimerGroup::new(peripherals.TIMG0);
    let _init = esp_wifi::init(
        timer1.timer0,
        esp_hal::rng::Rng::new(peripherals.RNG),
        peripherals.RADIO_CLK,
    )
    .unwrap();

    // Initialize GPIO pins
    let high_water_sensor = Input::new(
        peripherals.GPIO4,
        esp_hal::gpio::InputConfig::default().with_pull(esp_hal::gpio::Pull::Down),
    );

    let drain_pump = Output::new(
        peripherals.GPIO6,
        esp_hal::gpio::Level::Low,
        esp_hal::gpio::OutputConfig::default(),
    );

    let heartbeat_led = Output::new(
        peripherals.GPIO15,
        esp_hal::gpio::Level::Low,
        esp_hal::gpio::OutputConfig::default(),
    );

    // Spawn tasks
    spawner
        .spawn(check_container_level(high_water_sensor, drain_pump))
        .unwrap();

    spawner.spawn(heartbeat(heartbeat_led)).unwrap();

    // Wait indefinitely - tasks are running independently
    Timer::after(Duration::MAX).await;
}

#[embassy_executor::task]
async fn heartbeat(mut led: Output<'static>) {
    loop {
        info!("Heartbeat LED on");
        led.set_high();
        Timer::after(Duration::from_secs(1)).await;
        info!("Heartbeat LED off");
        led.set_low();
        Timer::after(Duration::from_secs(5)).await;
    }
}

#[embassy_executor::task]
async fn check_container_level(high_water_sensor: Input<'static>, mut drain_pump: Output<'static>) {
    loop {
        let high_water = high_water_sensor.is_high();

        if high_water && drain_pump.is_set_low() {
            info!("High water level detected - activating drain pump");
            drain_pump.set_high();
        } else if !high_water && drain_pump.is_set_high() {
            info!("Water level normalized - deactivating drain pump");
            drain_pump.set_low();
        }

        // Check every 10 seconds
        Timer::after(Duration::from_secs(10)).await;
    }
}
