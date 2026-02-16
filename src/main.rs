use pyo3::prelude::*;
use pyo3::types::PyModule;
use std::time::Duration;
use std::sync::mpsc;
use std::thread;
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

    //thread safe channel for Python -> Rust communication
    let (tx, rx) = mpsc::channel::<String>();

    // 2. Spawn the background thread for Python CV
    // We use a handle to join it later, ensuring it finishes cleanup.
    let python_thread_handle = thread::spawn(move || {
        Python::with_gil(|py| -> PyResult<()> {
            // -- PATH SETUP (Must be first) --
            let sys = py.import("sys")?;
            let path = sys.getattr("path")?;
            path.call_method1("append", ("./venv/Lib/site-packages",))?;
            path.call_method1("append", (".",))?;

            // -- MODULE LOADING --
            let file_path = "ThreadHandler.py";
            let py_code_raw = fs::read_to_string(file_path).expect("Failed to read Python file");
            let py_code_c = CString::new(py_code_raw).unwrap();
            let module_name = CStr::from_bytes_with_nul(b"ThreadHandler\0").unwrap();
            let file_name = CStr::from_bytes_with_nul(b"ThreadHandler.py\0").unwrap();

            let th_module = PyModule::from_code(
                py, 
                py_code_c.as_c_str(), 
                file_name,
                module_name
            )?;

            // -- INITIALIZE PYTHON CLASS --
            // Calling debug() sets up the instance and starts internal threads
            let handler_instance = th_module.getattr("debug")?.call0()?;

            // -- POLL LOOP (Background Thread) --
            loop {
                // Check if Python has new results
                let result_bound = handler_instance.call_method0("get_latest_results")?;
                
                if !result_bound.is_none() {
                    let data: String = result_bound.extract()?;
                    if tx.send(data).is_err() {
                        break;
                    } // Main thread closed channel
                }

                // Small sleep to release the GIL for internal Python threads
                thread::sleep(Duration::from_millis(10));
            }

            // --- SHUTDOWN PHASE INSIDE THREAD ---
            // This is reached if the loop breaks (e.g., tx drops)
            let _ = handler_instance.call_method0("stop");
            Ok(())
        }).expect("Python thread crashed");
    });

    println!("entering Loop");
    
    //break out of main control loop with 'E' key
    let mut i: u32 = 0;
    loop {
        i += 1;
        // println!("i: {}", i);
        input.update();

        // 5. NON-BLOCKING CHECK FOR CV DATA
        // try_recv() returns immediately. It doesn't wait.
        while let Ok(cv_data) = rx.try_recv() {
            // Update your robot's state with cv_data here
            println!("New CV target: {}", cv_data);
        }

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
    
    // A. Stop hardware immediately
    drivetrain::apply_inputs(&mut drivetrain_motors, 0.0, 0.0, 0.0);

    // C. Wait for the Python thread to finish cleaning up (releasing camera, joining threads)
    let _ = python_thread_handle.join();
    
    println!("Shutdown complete.");
    
    Ok(())
}
