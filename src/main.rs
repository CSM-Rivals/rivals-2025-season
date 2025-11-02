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

    let mut drivetrain_motors: Motors = Motors::new(&gpio, 5, 6, 13, 19);
    // let mut shooter = Shooter::new(&gpio, 24);
    // let mut intake = Intake::new(gpio, 23);

    let mut input = Input::new();

    println!("Done initializing, waiting for ESC startup...");
    // drivetrain_motors.startup();
    println!("ESC startup finished");

    // drivetrain::apply_inputs(&mut drivetrain_motors, 0.0, 0.25);

    // shooter.update(true);
    println!("asdf");

    let mut i: u32 = 0;
    loop {
        i += 1;
        // println!("i: {}", i);
        input.update();

        // println!("asdf");
        // println!("x: {}", input.state.x);
        drivetrain::apply_inputs(&mut drivetrain_motors, input.state.left_stick_x, input.state.left_stick_y);

        // shooter.update(true);
        // shooter.update(input.state.x);

        std::thread::sleep(Duration::from_millis(10));
    }
}
