/* Simple Rust program that prints a message and blinks the LEDs on stm32f767 board
*/

#![no_std]
#![no_main]

use panic_probe as _;
use cortex_m as _;
use cortex_m_rt::entry;
use stm32f7xx_hal::{pac, prelude::*};
use fugit::HertzU32;
use cortex_m_semihosting::{hprintln};

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    let cp = cortex_m::Peripherals::take().unwrap();

    // Configure Rcc for clocking
    let rcc = dp.RCC.constrain();
    let clk_rate = HertzU32::MHz(16); // 16 MHz
    let clocks = rcc.cfgr.sysclk(clk_rate).freeze();

    // Configure GPIO for LEDs (all 3 are on GPIOB)
    let gpiob = dp.GPIOB.split();
    let green_pin = gpiob.pb0; //  LED on PB0 - GREEN
    let blue_pin = gpiob.pb7;  //  LED on PB7 - BLUE
    let red_pin = gpiob.pb14; //  LED on PB14 - RED
    let mut green = green_pin.into_push_pull_output(); // set to MODE: OUTPUT
    let mut blue = blue_pin.into_push_pull_output();
    let mut red = red_pin.into_push_pull_output();

    // Configure SysTick for delay
    let mut delay = cp.SYST.delay(&clocks);

    hprintln!("\nHello, stm32 RUST world!");
    let stagger: u32 = 50;
    loop {
        green.set_high();
        delay.delay_ms(stagger);
        blue.set_high();
        delay.delay_ms(stagger);
        red.set_high();
        delay.delay_ms(500_u32);

        red.set_low();
        delay.delay_ms(stagger);
        blue.set_low();
        delay.delay_ms(stagger);
        green.set_low();
        delay.delay_ms(500_u32);
    }
}
