# Rust on Arduino UNO

> How to write a Rust Embedded-HAL for Arduino

This repo uses Arduino as an example to illustrate what steps are needed to run Rust on a microcontroller.

> A Rust package is also called a `crate`.

- **Register Level**: Also known as **PAC** (Peripheral Access Crates), where we work directly with registers.
- **Chip Level**: Also known as **HAL** (Hardware Abstraction Layer), where we can use functions (SDK) to access the hardware.
- **Board Level**: Also know as **BSP** (Board Support Package), where we support different boards that have the same chip.

Taking `Arduino Uno` as an example, the following `crates` make it possible to use Rust on AVR microcontrollers.

- **Register Level**: [`avr-device`](https://crates.io/crates/avr-device)
- **Chip Level**: [`avr-hal`](https://github.com/Rahix/avr-hal) (`attiny-hal` and `atmega-hal`)
- **Board Level**: [`arduino-hal`](https://github.com/Rahix/avr-hal/tree/main/arduino-hal)

> Acknowledgement: [`Rahix`](https://github.com/Rahix/) did a lot of work to make Rust on Arduino possible.

For this repo, you can find the code at different level in the following folders.

- **Register Level**: [`hello-avr-device`](hello-avr-device/)
- **Chip Level**: [`hello-atmega-hal`](hello-atmega-hal/)
- **Board Level**: [`hello-arduino-hal`](hello-arduino-hal/)

Below is an overview of how the code gets more human-readable as we go from the bottom level to the top level.

The register level:

```Rust
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
```

The chip level:

```Rust
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
```

The board level:

```Rust
#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let mut led = pins.d13.into_output();

    loop {
        led.toggle();
        arduino_hal::delay_ms(1000);
    }
}
```

## Prerequisites

First of all, we need a compiler. By default, the Rust toolchain cannot generate AVR bytecode and the AVR support was added recently, so we need a `nightly` toolchain.

```
$ rustup toolchain install nightly
```

We also need the `avr-gcc` toolchain. (This may not be needed in the future).

Ubuntu Users:

```
$ sudo apt-get install gcc-avr
```

Windows Users:

- AVR-GCC: https://www.microchip.com/en-us/tools-resources/develop/microchip-studio/gcc-compilers

> Don't forget to add the folder to `avr8-gnu-toolchain-win32_x86_64\bin` environment path.

Next, we need the tool to flash the firmware. You may have heard of [`avrdude`](https://github.com/avrdudes/avrdude), the programmer for AVR microcontrollers.

However, it can be tedious to find the firmware built by Rust and flash it to the hardware. To make things easier, `ravedude` allows you to use a single command `cargo run` to build and deploy the firmware to the hardware.

```
$ cargo install ravedude
```

Let's tests it out. The command `cargo run` uses `ravedude` to invoke `avrdude` to flash the firmware.

```
$ git clone https://github.com/wuhanstudio/arduino-uno-rust-avr-hal
$ cd arduino-uno-rust-avr-hal/hello-arduino-hal
$ cargo build
$ cargo run
```

Please rembember to change the board info in `Ravedude.toml`, especially the `port`:

```
[general]
board = "uno"
# After flashing, open the serial console at 57600 baud.
open-console = true
serial-baudrate = 115200
port = "COM7"
```

If you can see the LED blinking, we are ready to go.

## Step 1: Register Level

```
$ cargo install -f atdf2svd
$ cargo install svd2rust
$ cargo install form
```

## Step 2: Chip Level

```
$ rustup component add llvm-tools-preview
$ cargo size
```

## Step 3: Board Level



## Generate a new project

You can use `cargo-generate` to create a new empty project.

```
$ cargo install cargo-generate
$ cargo install ravedude
$ cargo generate --git https://github.com/Rahix/avr-hal-template.git
```
