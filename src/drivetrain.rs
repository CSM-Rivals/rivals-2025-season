use rppal::gpio::Gpio;
use uom::si::angular_velocity::{revolution_per_minute, AngularVelocity};

use crate::hardware::motor::{OpenLoopMotor, PWMMotor};

const SQRT2_2: f64 = 0.70710678118;

pub struct Motors {
    bl: PWMMotor,
    br: PWMMotor,
    fl: PWMMotor,
    fr: PWMMotor,
}

impl Motors {
    pub fn new(gpio: Gpio, bl_pin: u8, br_pin: u8, fl_pin: u8, fr_pin: u8) -> Motors {
        Motors {
            bl: PWMMotor::new(gpio.get(bl_pin).unwrap().into_output(), AngularVelocity::new::<revolution_per_minute>(1000.0)),
            br: PWMMotor::new(gpio.get(br_pin).unwrap().into_output(), AngularVelocity::new::<revolution_per_minute>(1000.0)),
            fl: PWMMotor::new(gpio.get(fl_pin).unwrap().into_output(), AngularVelocity::new::<revolution_per_minute>(1000.0)),
            fr: PWMMotor::new(gpio.get(fr_pin).unwrap().into_output(), AngularVelocity::new::<revolution_per_minute>(1000.0)),
        }
    }
}

pub fn apply_inputs(motors: &mut Motors, x: f64, y: f64) {
    motors.bl.set_power(-x * SQRT2_2 + y * SQRT2_2);
    motors.br.set_power(x * SQRT2_2 + y * SQRT2_2);
    motors.fl.set_power(x * SQRT2_2 + y * SQRT2_2);
    motors.fr.set_power(-x * SQRT2_2 + y * SQRT2_2);
}