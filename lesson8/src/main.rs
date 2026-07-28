#![no_std]
#![no_main]

use panic_halt as _;
use embedded_hal::digital::PinState;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let switch = pins.d2.into_pull_up_input();

    let mut led = pins.d13.into_output();

    loop {
        if switch.is_high() {led.set_high();} else {led.set_low();};
    }
}
