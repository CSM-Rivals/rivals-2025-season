use rppal::gpio::Gpio;
use uom::si::angular_velocity::{AngularVelocity, revolution_per_minute};

use crate::hardware::motor::{OpenLoopMotor, PWMMotor};

pub struct Shooter {
    motor: PWMMotor,
}

impl Shooter {
    pub fn new(gpio: &Gpio, pin: u8, reverse_pin: u8) -> Shooter {
        println!("Shooter startup");
        Shooter {
            motor: PWMMotor::new(gpio.get(pin).unwrap().into_output(), Some(gpio.get(reverse_pin).unwrap().into_output()), AngularVelocity::new::<revolution_per_minute>(1000.0)),
        }
    }

    pub fn update(&mut self, run: bool) {
        if run {
            self.motor.set_power(0.5);
        } else {
            self.motor.set_power(0.0);
        }
    }
}