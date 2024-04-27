use std::error::Error;
use std::thread;
use std::time::Duration;

use rppal::gpio::Gpio;

const GPIO_PWM: [u8; 20] = [
    10, 9, 11, 5, 13, 19, 18, 23, 24, 25, 8, 7, 1, 12, 20, 21, 4, 27, 6, 16,
];

// const PERIOD_MS: u64 = 60; //20
// const PULSE_MIN_US: u64 = 500; // 1200
// const PULSE_NEUTRAL_US: u64 = 3500; // 1500
// const PULSE_MAX_US: u64 = 5500; //1800

pub fn test_servo(
    period_ms: u64,
    pulse_min_us: u64,
    pulse_neutral_us: u64,
    pulse_max_us: u64,
) -> Result<(), Box<dyn Error>> {
    for gpio_number in GPIO_PWM {
        println!("Test gpio: {gpio_number}");
        let mut pin = Gpio::new()?.get(gpio_number)?.into_output();

        pin.set_pwm(
            Duration::from_millis(period_ms),
            Duration::from_micros(pulse_max_us),
        )?;

        thread::sleep(Duration::from_millis(500));

        pin.set_pwm(
            Duration::from_millis(period_ms),
            Duration::from_micros(pulse_min_us),
        )?;

        thread::sleep(Duration::from_millis(500));

        for pulse in (pulse_min_us..=pulse_neutral_us).step_by(10) {
            pin.set_pwm(
                Duration::from_millis(period_ms),
                Duration::from_micros(pulse),
            )?;
            thread::sleep(Duration::from_millis(20));
        }
        println!("Done");
    }

    Ok(())
}
