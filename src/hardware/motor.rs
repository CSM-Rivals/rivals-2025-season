use std::time::Duration;
#[cfg(target_os = "linux")]
use rppal::gpio::OutputPin;
#[cfg(target_os = "linux")]
use uom::si::f64::*;

pub trait OpenLoopMotor {
    fn set_power(&mut self, power: f64);
    #[cfg(target_os = "linux")]
    fn set_velocity(&mut self, velocity: AngularVelocity);
    #[cfg(not(target_os = "linux"))]
    fn set_velocity(&mut self, velocity: f64);
}

#[cfg(target_os = "linux")]
pub struct PWMMotor {
    pin: OutputPin,
    reverse_pin: Option<OutputPin>,
    period: Duration,
    min_pulse_width: Duration,
    max_pulse_width: Duration,
    max_velocity: AngularVelocity,

    power: f64,
}

#[cfg(target_os = "linux")]
impl PWMMotor {
    pub fn new(
        pin: OutputPin,
        reverse_pin: Option<OutputPin>,
        max_velocity: AngularVelocity,
    ) -> PWMMotor {
        let mut motor = PWMMotor {
            pin: pin,
            reverse_pin: reverse_pin,
            period: Duration::from_millis(20),
            min_pulse_width: Duration::from_millis(1),
            max_pulse_width: Duration::from_millis(2),
            max_velocity: max_velocity,

            power: 0.
        };

        motor.set_power(0.0);

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

#[cfg(target_os = "linux")]
impl OpenLoopMotor for PWMMotor {
    fn set_power(&mut self, power: f64) {
        match &mut self.reverse_pin {
            Some(reverse_pin) => {
                let _ = self.pin.set_pwm(
                    self.period,
                    (self.max_pulse_width - self.min_pulse_width).mul_f64(power.abs()) + self.min_pulse_width,
                );

                if power < 0.0 {
                    let _ = reverse_pin.set_high();
                } else {
                    let _ = reverse_pin.set_low();
                }
            },
            None => {
                let _ = self.pin.set_pwm(
                    self.period,
                    Duration::from_secs_f64((self.max_pulse_width.as_secs_f64() - self.min_pulse_width.as_secs_f64()) * ((power + 1.0) / 2.0) + self.min_pulse_width.as_secs_f64()),
                );
            },
        }

        self.power = power;
    }

    fn set_velocity(&mut self, velocity: AngularVelocity) {
        self.set_power((velocity / self.max_velocity).value);
    }
}

//Below is SIM implementation for windows when the linux os from the pi is not targeted
//TODO: change printlines to update an actual simulation

#[cfg(not(target_os = "linux"))]
pub struct PWMMotor {
    _pin_info: String,
    power: f64,
}

#[cfg(not(target_os = "linux"))]
impl PWMMotor {
    // We change the arguments to types that exist on Windows (u8 instead of OutputPin)
    pub fn new(_pin: u8, _reverse_pin: Option<u8>, _max_velocity: f64) -> PWMMotor {
        println!("SIM: PWMMotor initialized on Windows");
        PWMMotor {
            _pin_info: format!("Pin {}", _pin),
            power: 0.0,
        }
    }

    pub fn set_pwm_config(&mut self, _: Duration, _: Duration, _: Duration) {
        println!("SIM: PWM Config updated");
    }
}

#[cfg(not(target_os = "linux"))]
impl OpenLoopMotor for PWMMotor {
    fn set_power(&mut self, power: f64) {
        self.power = power;
        // println!("SIM: Setting power to {}", power);    //TODO: uncomment
    }

    fn set_velocity(&mut self, velocity: f64) {
        println!("SIM: Setting velocity to {}", velocity);
    }
}
