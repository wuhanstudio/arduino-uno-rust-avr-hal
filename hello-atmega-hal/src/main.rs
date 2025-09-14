#![no_std]
#![no_main]

use panic_halt as _;

use atmega_hal::clock::MHz16;
use embedded_hal::delay::DelayNs;

#[avr_device::entry]
fn main() -> !{
    let dp = atmega_hal::Peripherals::take().unwrap();

    let pins = atmega_hal::pins!(dp);
    let mut led = pins.pb5.into_output();

    let mut delay = atmega_hal::delay::Delay::<MHz16>::new();

    loop {
        led.toggle();
        delay.delay_ms(1000);
    }
}
