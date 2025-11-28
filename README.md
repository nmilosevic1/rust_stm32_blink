# rust_stm32_blink
First Embedded Rust Project

This book is very helpful for getting your environment setup:
https://docs.rust-embedded.org/book/intro/install.html?highlight=rustup#rust-toolchain
  The book is for a different board, but it was very easy to setup for my board instead.

This project blinks the LEDs and prints Hello World.

Be sure to update the values in memory.x to work with your board.
e.g. For stm32f767 the reference manual has these values in section
2.2.2 Memory map and register boundary addresses
  Figure 2. Memory map
     which shows "512-Mbyte Block 1 SRAM" is at 0x2000 0000 - 0x3FFF FFFF
                             total 0x20000000 (536870912 decimal) bytes but part of that is reserved:
                             Reserved
                             SRAM2 (16 KB)
                             SRAM1 (368 KB)
                             DTCM (128 KB)    16+368+128 == 512K
     and "Flash memory on AXIM interface" is at 0x0800 0000 - 0x081F FFFF
                             total 0x200000 (2097152 decimal) bytes == 2048K


Once rust and all your tools are installed you might want to switch to the nightly:
 rustup update
 rustup install nightly
 rustup default nightly

Other stuff I installed:
 rustup target add thumbv7em-none-eabi
 rustup target add thumbv7em-none-eabihf
 cargo install cargo-binutils
 rustup component add llvm-tools
 cargo install cargo-generate
 cargo install flip-link
 cargo add fugit
 cargo add cortex_m_semihosting

Clean and build with:
 cargo clean
 cargo build

I am using openocd and arm-none-eabi-gdb to flash and debug:
* in one terminal start openocd
* in another terminal run:
 cargo run target/thumbv7em-none-eabihf/debug/blinky
 type 'c' to continue in the gdb window to run the program

The lights on the board will blink and the Hello World prints in the openocd terminal

