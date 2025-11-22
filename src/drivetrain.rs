use std::time::Duration;

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
    pub fn new(gpio: &Gpio, bl_pin: u8, br_pin: u8, fl_pin: u8, fr_pin: u8) -> Motors {
        Motors {
            bl: PWMMotor::new(gpio.get(bl_pin).unwrap().into_output(), None, AngularVelocity::new::<revolution_per_minute>(1000.0)),
            br: PWMMotor::new(gpio.get(br_pin).unwrap().into_output(), None, AngularVelocity::new::<revolution_per_minute>(1000.0)),
            fl: PWMMotor::new(gpio.get(fl_pin).unwrap().into_output(), None, AngularVelocity::new::<revolution_per_minute>(1000.0)),
            fr: PWMMotor::new(gpio.get(fr_pin).unwrap().into_output(), None, AngularVelocity::new::<revolution_per_minute>(1000.0)),
        }
    }

    pub fn startup(&mut self) {
        std::thread::sleep(Duration::from_secs(2));

        println!("set half");
        self.bl.set_power(0.5);
        self.br.set_power(0.5);
        self.fl.set_power(0.5);
        self.fr.set_power(0.5);
        
        std::thread::sleep(Duration::from_secs(2));

        println!("set zero");
        self.bl.set_power(0.0);
        self.br.set_power(0.0);
        self.fl.set_power(0.0);
        self.fr.set_power(0.0);

        std::thread::sleep(Duration::from_secs(2));
    }
}

pub fn apply_inputs(motors: &mut Motors, x: f64, y: f64, r: f64) {
    // motors.bl.set_power(y);
    // motors.br.set_power(y);
    // motors.fl.set_power(y);
    // motors.fr.set_power(y);
    motors.bl.set_power(x * SQRT2_2 + y * SQRT2_2 - r);
    motors.br.set_power(-x * SQRT2_2 + y * SQRT2_2 + r);
    motors.fl.set_power(x * SQRT2_2 + y * SQRT2_2 + r);
    motors.fr.set_power(-x * SQRT2_2 + y * SQRT2_2 - r);
}