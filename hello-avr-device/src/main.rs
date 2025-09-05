#![no_std]
#![no_main]

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    // disable interrupts - firmware has panicked so no ISRs should continue running
    avr_device::interrupt::disable();
    loop {
        
    }
}

// Never returns
#[avr_device::entry]
fn main() -> ! {
    let dp = avr_device::atmega328p::Peripherals::take().unwrap();

    dp.PORTB.ddrb.modify(|_, w| w.pb5().set_bit());

    loop {
        avr_device::asm::delay_cycles(10_000_000);
        dp.PORTB.portb.write(|w| w.pb5().set_bit());

        avr_device::asm::delay_cycles(10_000_000);
        dp.PORTB.portb.write(|w| w.pb5().clear_bit());
    }
}
