#![no_std]
#![no_main]

use panic_halt as _;
use arduino_hal::prelude::*;

const KEYPAD: [[&str; 4]; 4] = [["1", "2", "3", "A"],
                                ["4", "5", "6", "B"],
                                ["7", "8", "9", "C"],
                                ["*", "0", "#", "D"]];

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);

    let mut rows = (pins.d9.into_output(), pins.d8.into_output(), pins.d7.into_output(), pins.d6.into_output());
    let columns = (pins.d5.into_pull_up_input(), pins.d4.into_pull_up_input(), pins.d3.into_pull_up_input(), pins.d2.into_pull_up_input());

    rows.0.set_high();
    rows.1.set_high();
    rows.2.set_high();
    rows.3.set_high();

    let wait_for_release = || {
        loop {
            if columns.0.is_high() && columns.1.is_high() && columns.2.is_high() && columns.3.is_high() {
                break;
            }
        }
    };

    loop {
        rows.0.set_low();
        match (columns.0.is_low(), columns.1.is_low(), columns.2.is_low(), columns.3.is_low()) {
            (true, _, _, _) => {
                ufmt::uwriteln!(&mut serial, "{}", KEYPAD[0][0]).unwrap_infallible();
                wait_for_release();
                rows.0.set_high();
                continue;
            },
            (false, true, _, _) => {
                ufmt::uwriteln!(&mut serial, "{}", KEYPAD[0][1]).unwrap_infallible();
                wait_for_release();
                rows.0.set_high();
                continue;
            },
            (false, false, true, _) => {
                ufmt::uwriteln!(&mut serial, "{}", KEYPAD[0][2]).unwrap_infallible();
                wait_for_release();
                rows.0.set_high();
                continue;
            },
            (false, false, false, true) => {
                ufmt::uwriteln!(&mut serial, "{}", KEYPAD[0][3]).unwrap_infallible();
                wait_for_release();
                rows.0.set_high();
                continue;
            },
            (false, false, false, false) => rows.0.set_high(),
        }

        rows.1.set_low();
        match (columns.0.is_low(), columns.1.is_low(), columns.2.is_low(), columns.3.is_low()) {
            (true, _, _, _) => {
                ufmt::uwriteln!(&mut serial, "{}", KEYPAD[1][0]).unwrap_infallible();
                wait_for_release();
                rows.1.set_high();
                continue;
            },
            (false, true, _, _) => {
                ufmt::uwriteln!(&mut serial, "{}", KEYPAD[1][1]).unwrap_infallible();
                wait_for_release();
                rows.1.set_high();
                continue;
            },
            (false, false, true, _) => {
                ufmt::uwriteln!(&mut serial, "{}", KEYPAD[1][2]).unwrap_infallible();
                wait_for_release();
                rows.1.set_high();
                continue;
            },
            (false, false, false, true) => {
                ufmt::uwriteln!(&mut serial, "{}", KEYPAD[1][3]).unwrap_infallible();
                wait_for_release();
                rows.1.set_high();
                continue;
            },
            (false, false, false, false) => rows.1.set_high(),
        }

        rows.2.set_low();
        match (columns.0.is_low(), columns.1.is_low(), columns.2.is_low(), columns.3.is_low()) {
            (true, _, _, _) => {
                ufmt::uwriteln!(&mut serial, "{}", KEYPAD[2][0]).unwrap_infallible();
                wait_for_release();
                rows.2.set_high();
                continue;
            },
            (false, true, _, _) => {
                ufmt::uwriteln!(&mut serial, "{}", KEYPAD[2][1]).unwrap_infallible();
                wait_for_release();
                rows.2.set_high();
                continue;
            },
            (false, false, true, _) => {
                ufmt::uwriteln!(&mut serial, "{}", KEYPAD[2][2]).unwrap_infallible();
                wait_for_release();
                rows.2.set_high();
                continue;
            },
            (false, false, false, true) => {
                ufmt::uwriteln!(&mut serial, "{}", KEYPAD[2][3]).unwrap_infallible();
                wait_for_release();
                rows.2.set_high();
                continue;
            },
            (false, false, false, false) => rows.2.set_high(),
        }

        rows.3.set_low();
        match (columns.0.is_low(), columns.1.is_low(), columns.2.is_low(), columns.3.is_low()) {
            (true, _, _, _) => {
                ufmt::uwriteln!(&mut serial, "{}", KEYPAD[3][0]).unwrap_infallible();
                wait_for_release();
                rows.3.set_high();
                continue;
            },
            (false, true, _, _) => {
                ufmt::uwriteln!(&mut serial, "{}", KEYPAD[3][1]).unwrap_infallible();
                wait_for_release();
                rows.3.set_high();
                continue;
            },
            (false, false, true, _) => {
                ufmt::uwriteln!(&mut serial, "{}", KEYPAD[3][2]).unwrap_infallible();
                wait_for_release();
                rows.3.set_high();
                continue;
            },
            (false, false, false, true) => {
                ufmt::uwriteln!(&mut serial, "{}", KEYPAD[3][3]).unwrap_infallible();
                wait_for_release();
                rows.3.set_high();
                continue;
            },
            (false, false, false, false) => rows.3.set_high(),
        }
    }
}
