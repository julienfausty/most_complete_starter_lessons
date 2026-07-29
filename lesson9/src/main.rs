#![no_std]
#![no_main]

use panic_halt as _;

const WAVE_LENGTH: u32 = 20000;
const LOW_PULSE: u32 = 500;
const HIGH_PULSE: u32 = 2500;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let mut servo = pins.d9.into_output();

    let n_steps = 10;
    let pulse_step = (HIGH_PULSE - LOW_PULSE) / n_steps;
    loop {
        for i_duty in (0..=n_steps).chain((1..n_steps).rev()) {
            for _ in 0..5 {
                let pulse = LOW_PULSE + i_duty * pulse_step;
                servo.set_high();
                arduino_hal::delay_us(pulse);
                servo.set_low();
                arduino_hal::delay_us(WAVE_LENGTH - pulse);
            }
            arduino_hal::delay_ms(1000);
        }
    }
}
