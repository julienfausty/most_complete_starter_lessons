#![no_std]
#![no_main]

use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let mut led = pins.d5.into_output();

    let input_on = pins.d9.into_pull_up_input();
    let input_off = pins.d8.into_pull_up_input();

    loop {
        if input_on.is_low() {
            led.set_high();
        }
        if input_off.is_low() {
            led.set_low();
        }
    }
}
