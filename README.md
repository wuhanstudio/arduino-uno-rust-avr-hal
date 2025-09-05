# Rust on Arduino UNO

## Quick Start

`ravedude` allows you to use `cargo run` for building, deploying, and running the AVR code.

```
cargo install ravedude
```

```
$ git clone https://github.com/wuhanstudio/arduino-uno-rust-avr-hal
$ cd arduino-uno-rust-avr-hal/hello-led
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


## Generate a new project

You can use `cargo-generate` to create a new empty project.

```
$ cargo install cargo-generate
$ cargo install ravedude
$ cargo generate --git https://github.com/Rahix/avr-hal-template.git
```
