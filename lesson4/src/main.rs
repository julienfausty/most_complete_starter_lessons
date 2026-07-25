#![no_std]
#![no_main]

use panic_halt as _;
use arduino_hal::simple_pwm::{IntoPwmPin, PwmPinOps, Prescaler};

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let mut pwm_timer0 = arduino_hal::simple_pwm::Timer0Pwm::new(dp.TC0, Prescaler::Direct);

    let mut pwm_timer2 = arduino_hal::simple_pwm::Timer2Pwm::new(dp.TC2, Prescaler::Direct);

    let mut blue_led = pins.d3.into_output().into_pwm(&pwm_timer2);
    let mut green_led = pins.d5.into_output().into_pwm(&pwm_timer0);
    let mut red_led = pins.d6.into_output().into_pwm(&pwm_timer0);

    blue_led.enable();
    green_led.enable();
    red_led.enable();

    blue_led.set_duty(0);
    green_led.set_duty(200);
    red_led.set_duty(0);

    loop {
        red_led.set_duty(0);
        blue_led.set_duty(255);
        green_led.set_duty(0);
        arduino_hal::delay_ms(1000);
        red_led.set_duty(100);
        blue_led.set_duty(100);
        green_led.set_duty(100);
        arduino_hal::delay_ms(1000);
        red_led.set_duty(255);
        blue_led.set_duty(0);
        green_led.set_duty(0); 
        arduino_hal::delay_ms(1000);
    }
}
