#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::{delay::Delay, gpio::Io, i2c::I2c, prelude::*};

#[entry]
fn main() -> ! {
    #[allow(unused)]
    let peripherals = esp_hal::init(esp_hal::Config::default());
    let delay = Delay::new();

    esp_println::logger::init_logger_from_env();

    let io = Io::new(peripherals.GPIO, peripherals.IO_MUX);

    let sda = io.pins.gpio19;
    let scl = io.pins.gpio18;
    let i2c = I2c::new(peripherals.I2C0, sda, scl, 100.kHz());

    let mut battery = max17048::Max17048::new(i2c);

    loop {
        let charge = battery.soc().unwrap();
        log::info!("MAX17043 charge: {}%", charge);
        delay.delay(500.millis());
    }
}
