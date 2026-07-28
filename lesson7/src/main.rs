#![no_std]
#![no_main]

use panic_halt as _;

const FREQUENCIES: [u32; 8] = [523, 587, 659, 698, 784, 880, 998, 1047];

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let mut buzzer = pins.d8.into_output();

    loop {
        for frequency in FREQUENCIES.iter() {
            let delay_in_us = (1000000 / frequency) / 2;
            for _ in 0..(2*frequency) {
                buzzer.toggle();
                arduino_hal::delay_us(delay_in_us);
            }
        }
        arduino_hal::delay_ms(2000);
    }
}
