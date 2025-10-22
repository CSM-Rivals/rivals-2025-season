use core::time::Duration;

use rppal::gpio::Gpio;
use uom::si::angular_velocity::{revolution_per_minute, AngularVelocity};

use crate::hardware::motor::PWMMotor;
use crate::hardware::motor::OpenLoopMotor;

mod hardware;
mod sm;

fn main() {
    println!("start");


    let gpio = Gpio::new().unwrap();
    let pin = gpio.get(26).unwrap().into_output();

    let mut motor = PWMMotor::new(pin, AngularVelocity::new::<revolution_per_minute>(1000.));
    std::thread::sleep(Duration::from_secs(10));

    println!("set power");
    motor.set_power(0.5);
    std::thread::sleep(Duration::from_secs(5));

    let mut power: f64 = 0.;
    loop {
        power += 0.01;
        if power == 1. {
            power = -1.;
        }

        motor.set_power(power);

        std::thread::sleep(Duration::from_millis(50));
    }

    // loop {
    //     println!("on");
    //     let _ = pin.set_pwm(Duration::from_millis(20), Duration::from_micros(1500));
    //     std::thread::sleep(Duration::from_secs(5));
    //     println!("off");
    //     let _ = pin.set_pwm(Duration::from_millis(20), Duration::from_micros(1600));
    //     std::thread::sleep(Duration::from_secs(5));
    // }
}
