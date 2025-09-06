#![no_std]
#![no_main]

use panic_halt as _;
use embassy_executor::Spawner;

// Not Implemented
// use embassy_time::{Duration, Timer};

use arduino_hal::port::Pin;
use arduino_hal::port::D13;
use arduino_hal::port::mode::Output;

use arduino_hal::port::D0;
use arduino_hal::port::D1;
use atmega_hal::pac::USART0;
use arduino_hal::port::mode::Input;

// Task 1: Blink the LED
#[embassy_executor::task]
async fn blinker(mut led: Pin<Output, D13>, interval: u32) {
    loop {
        led.toggle();
        arduino_hal::delay_ms(interval);

        // Not Implemented
        // Timer::after(Duration::from_millis(500)).await;
    }
}

// Task 2: Serial output
#[embassy_executor::task]
async fn talk(mut serial: arduino_hal::Usart<USART0, Pin<Input, D0>, Pin<Output, D1>>, interval: u32) -> ! {
    loop {
        ufmt::uwriteln!(&mut serial, "Hello from Task!\r").unwrap();
        arduino_hal::delay_ms(interval);

        // Not Implemented
        // Timer::after(Duration::from_millis(500)).await;
    }
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    // Task 1: Blink the LED
    let led = pins.d13.into_output();
    spawner.spawn(blinker(led, 300)).unwrap();

    // Task 2: Serial output
    let serial = arduino_hal::default_serial!(dp, pins, 57600);
    spawner.spawn(talk(serial, 1000)).unwrap();
}
