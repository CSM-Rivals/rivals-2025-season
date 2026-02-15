use pyo3::prelude::*;
use pyo3::types::PyModule;
use std::time::Duration;
use std::fs;
use::std::ffi::CStr;
use std::ffi::CString;
use device_query::{DeviceQuery, DeviceState, Keycode};
#[cfg(target_os = "linux")]
use rppal::gpio::Gpio;

use crate::{comms::Input, drivetrain::Motors, intake::Intake, shooter::Shooter};

mod hardware;
mod drivetrain;
mod shooter;
mod intake;
mod comms;

fn main() -> PyResult<()> {
    println!("Initializing");
    #[cfg(target_os = "linux")]
    println!("Initializing GPIO for Raspberry Pi...");

    #[cfg(target_os = "linux")]
    let gpio = Gpio::new().unwrap();
    #[cfg(not(target_os = "linux"))]
    let gpio = {
        println!("Raspberry Pi not found, skipping GPIO initialization.");
        0u8
    };

    let mut drivetrain_motors: Motors = Motors::new(&gpio, 5, 6, 13, 19);
    let mut shooter = Shooter::new(&gpio, 24, 27);
    let mut intake = Intake::new(&gpio, 23, 22);

    let mut input = Input::new();

    let device_state = DeviceState::new();

    println!("Done initializing, waiting for ESC startup...");
    // drivetrain_motors.startup();
    println!("ESC startup finished");
    // drivetrain::apply_inputs(&mut drivetrain_motors, 0.0, 0.25);
    // shooter.update(true);
    println!("asdf");

    //initialize python interperter and GIL
    Python::with_gil(|py| -> PyResult<()>{

        //add the site-packages and cuurent directory to path
        let sys = py.import("sys")?;
        let path = sys.getattr("path")?;
        path.call_method1("append", ("./venv/Lib/site-packages",))?;
        path.call_method1("append", (".",))?;

        //read file as c language string (raw -> c string)
        let file_path = "ThreadHandler.py";
        let py_code_raw = fs::read_to_string(file_path).map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Null byte in Python code: {}", e))
        })?;
        let py_code_c = CString::new(py_code_raw).map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Null byte in Python code: {}", e))
        })?;

        let module_name = CStr::from_bytes_with_nul(b"ThreadHandler\0").unwrap();
        let file_name = CStr::from_bytes_with_nul(b"ThreadHandler.py\0").unwrap();

        let th_module = PyModule::from_code(
            py, 
            py_code_c.as_c_str(), 
            file_name,
            module_name
        )?;

        //run python function (currently access_python)
        let run_func = th_module.getattr("debug")?;

        //get return value from python function as string
        let result: String = run_func.call0()?.extract()?;

        println!("Rust received: {}", result);

        Ok(())
    })?;

    //break out of main control loop with 'E' key
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

        let keys: Vec<Keycode> = device_state.get_keys();
        if keys.contains(&Keycode::E) {
            break;
        }

        std::thread::sleep(Duration::from_millis(10));
    }
    Ok(())
}
