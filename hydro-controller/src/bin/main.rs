#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use esp_backtrace as _;
use esp_hal::clock::CpuClock;
use esp_hal::gpio::{Input, InputConfig, Level, Output, OutputConfig, Pull};
use esp_hal::timer::systimer::SystemTimer;
use esp_hal::timer::timg::TimerGroup;
use log::info;

extern crate alloc;

#[esp_hal_embassy::main]
async fn main(spawner: Spawner) {
    // generator version: 0.3.1

    esp_println::logger::init_logger_from_env();

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    esp_alloc::heap_allocator!(size: 72 * 1024);

    let timer0 = SystemTimer::new(peripherals.SYSTIMER);
    esp_hal_embassy::init(timer0.alarm0);

    info!("Embassy initialized!");

    let timer1 = TimerGroup::new(peripherals.TIMG0);
    let _init = esp_wifi::init(
        timer1.timer0,
        esp_hal::rng::Rng::new(peripherals.RNG),
        peripherals.RADIO_CLK,
    )
    .unwrap();

    // Initialize GPIO pins
    let mut led = Output::new(peripherals.GPIO15, Level::Low, OutputConfig::default());

    // Water level sensor pin (pulled down, will read high when water detected)
    let high_water_sensor = Input::new(
        peripherals.GPIO4,
        InputConfig::default().with_pull(Pull::Down),
    );

    // Pump control pin (default to off)
    let drain_pump = Output::new(peripherals.GPIO6, Level::Low, OutputConfig::default());

    // Spawn the water level monitoring task
    spawner
        .spawn(check_container_level(high_water_sensor, drain_pump))
        .unwrap();

    // LED heartbeat task
    loop {
        info!("Heartbeat");
        led.toggle();
        Timer::after(Duration::from_secs(15)).await;
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
