use dht_sensor::*;
use esp_idf_hal::delay::Delay;
use esp_idf_hal::peripherals::Peripherals;
use esp_idf_hal::gpio::PinDriver;
use esp_idf_svc::log::EspLogger;
use esp_idf_sys as _;
use log::info;

fn main() {
    esp_idf_sys::link_patches();
    EspLogger::initialize_default();

    info!("Hello, world!");

    let peripherals = Peripherals::take().unwrap();
    let mut dht_pin = PinDriver::input_output_od(peripherals.pins.gpio18).unwrap();

    // Pulling the pin high to avoid confusing the sensor when initializing
    dht_pin.set_high().ok();

    info!("Waiting on the sensor...");
    let mut delay: Delay = Default::default();
    // The DHT11 datasheet suggests 1 second
    delay.delay_ms(1000_u32);

    loop {
        match dht11::Reading::read(&mut delay, &mut dht_pin) {
            Ok(dht11::Reading {
                temperature,
                relative_humidity,
            }) => println!("{}°, {}% RH", temperature, relative_humidity),
            Err(e) => println!("Error {:?}", e),
        }

        delay.delay_ms(1000_u32);
    }
}