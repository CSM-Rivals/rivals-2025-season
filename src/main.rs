use pyo3::prelude::*;
use pyo3::types::{PyModule, PyAny};

use core::time::Duration;

use rppal::gpio::Gpio;

use crate::{comms::Input, drivetrain::Motors, intake::Intake, shooter::Shooter};

mod hardware;
mod drivetrain;
mod shooter;
mod intake;
mod comms;

fn main() -> PyResult<()> {
    println!("Initializing");

    let gpio = Gpio::new().unwrap();

    let mut drivetrain_motors: Motors = Motors::new(&gpio, 5, 6, 13, 19);
    let mut shooter = Shooter::new(&gpio, 24, 27);
    let mut intake = Intake::new(&gpio, 23, 22);

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
        drivetrain::apply_inputs(&mut drivetrain_motors, input.state.left_stick_x * -1.0, input.state.left_stick_y, input.state.right_stick_x * -0.5);

        // shooter.update(true);
        shooter.update(input.state.y);
        intake.update(input.state.x);

        std::thread::sleep(Duration::from_millis(10));
    }

    //initialize python interperter
    Python::with_gil(|py| { //get python Global Interperter Lock thread
        let sys = PyModule::import(py, "sys")?; //import python module "sys" to handle python parameters,
        //the question mark operator converts python exceptions into rust exceptions of PyModule objects
        let path = sys.getattr("C:\\Users\\nicho\\Documents\\Robotics\\rivals-2025-season")?; //get sys path
        path.call_method1("append", ("./",))?; //add current directory to path visible by python

        let py_module = PyModule::import(py, "ThreadHandler")?; //import ThreadHandler.py file
        let py_class = py_module.getattr("ThreadHandler")?; //get ThreadHandler class from imported file
        let py_class_instance = py_class.call0()?; //instantiate ThreadHandler as "ThreadHandler"

        instance.getattr("run")?.call0()?; //call the "run" method in ThreadHandler

        //let greeting_rust: String = greeting_py.extract()?;
        //println!("{}", greeting_rust); // Output: Hello, Rust User from Python!

        Ok(()) //close python interperter access
    })
}
