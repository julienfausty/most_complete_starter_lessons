#![no_std]
#![no_main]

use panic_halt as _;
use ufmt;
use arduino_hal::prelude::*;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);

    let mut trigger = pins.d12.into_output();
    let echo = pins.d11;

    let timer1 = dp.TC1; // 16Mhz
    // prescale TC1 timer to 1 / 4 us (250kHz)
    timer1.tccr1b().write(|w| w.cs1().prescale_64());

    'top: loop {
        // start timer
        timer1.tcnt1().write(|w| w.set(0));

        // send trigger pulse
        trigger.set_high();
        arduino_hal::delay_us(10);
        trigger.set_low();

        while echo.is_low() {
            // check if 200ms has passed
            if timer1.tcnt1().read().bits() >= 50000 {
                ufmt::uwriteln!(&mut serial, "Nothing detected in front of sensor.\r").unwrap_infallible();
                continue 'top;
            }
        }

        // restart timer to measure high pulse
        timer1.tcnt1().write(|w| w.set(0));

        while echo.is_high() {}

        let timed_microns = timer1.tcnt1().read().bits().saturating_mul(4);
        let timed_microns = match timed_microns {
            u16::MAX => {
                // If multiply is saturated high pulse is on all the time and no object is detected
                ufmt::uwriteln!(&mut serial, "Nothing detected in front of sensor.\r").unwrap_infallible();
                continue 'top;
            },
            _ => timed_microns,
        };

        let distance_in_cm = timed_microns / 58;

        ufmt::uwriteln!(&mut serial, "Object detected {} cm from sensor.\r", distance_in_cm).unwrap_infallible();

        // wait at least 60ms from high pulse to not have overlapping signals
        while timer1.tcnt1().read().bits() < 15000 {}
    }
}
