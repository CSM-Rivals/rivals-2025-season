use core::time::Duration;

use rppal::gpio::Gpio;

use crate::{comms::Input, drivetrain::Motors, intake::Intake, shooter::Shooter};

mod hardware;
mod drivetrain;
mod shooter;
mod intake;
mod comms;

fn main() {
    println!("Initializing");

    let gpio = Gpio::new().unwrap();

    // let mut drivetrain_motors: Motors = Motors::new(gpio, 5, 6, 13, 19);
    let mut shooter = Shooter::new(gpio, 24);
    // let mut intake = Intake::new(gpio, 23);

    let mut input = Input::new();

    println!("Done initializing, waiting for ESC startup...");
    std::thread::sleep(Duration::from_secs(10));
    println!("ESC startup finished");

    shooter.update(true);

    loop {
        input.update();

        println!("x: {}", input.state.x);
        // drivetrain::apply_inputs(&mut drivetrain_motors, 0.0, 0.0);

        // shooter.update(input.state.x);
        // intake.update(input.state.y);
    }
}
