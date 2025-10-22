use std::time::Duration;

use rppal::gpio::OutputPin;
use uom::si::f64::*;

pub trait OpenLoopMotor {
    fn set_power(&mut self, power: f64);
    fn set_velocity(&mut self, velocity: AngularVelocity);
}

pub struct PWMMotor {
    pin: OutputPin,
    period: Duration,
    min_pulse_width: Duration,
    max_pulse_width: Duration,
    max_velocity: AngularVelocity,

    power: f64,
}
impl PWMMotor {
    pub fn new(
        pin: OutputPin,
        max_velocity: AngularVelocity,
    ) -> PWMMotor {
        let mut motor = PWMMotor {
            pin: pin,
            period: Duration::from_millis(20),
            min_pulse_width: Duration::from_millis(1),
            max_pulse_width: Duration::from_millis(2),
            max_velocity: max_velocity,

            power: 0.
        };

        motor.set_power(0.);

        motor
    }

    pub fn set_pwm_config(
        &mut self,
        period: Duration,
        min_pulse_width: Duration,
        max_pulse_width: Duration,
    ) {
        self.period = period;
        self.min_pulse_width = min_pulse_width;
        self.max_pulse_width = max_pulse_width;

        self.set_power(self.power);
    }
}

impl OpenLoopMotor for PWMMotor {
    fn set_power(&mut self, power: f64) {
        let _ = self.pin.set_pwm(
            self.period,
            (self.max_pulse_width - self.min_pulse_width).mul_f64(power.abs()) + self.min_pulse_width,
        );

        self.power = power;
    }

    fn set_velocity(&mut self, velocity: AngularVelocity) {
        self.set_power((velocity / self.max_velocity).value);
    }
}
