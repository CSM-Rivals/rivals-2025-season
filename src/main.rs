use pyo3::prelude::*;
use pyo3::types::PyModule;
use std::time::Duration;
use std::time::Instant;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc;
use std::thread;
use std::fs;
use::std::ffi::CStr;
use std::ffi::CString;
use device_query::{DeviceQuery, DeviceState, Keycode};
use crossterm::event::{self, Event, KeyCode, KeyEvent};
#[cfg(target_os = "linux")]
use rppal::gpio::Gpio;

use crate::{comms::Input, drivetrain::Motors, intake::Intake, shooter::Shooter};

mod hardware;
mod drivetrain;
mod shooter;
mod intake;
mod comms;

fn main() -> PyResult<()> {
    std::env::set_var("PYTHONUNBUFFERED", "1");
    crossterm::terminal::enable_raw_mode().unwrap();
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

    // let device_state = DeviceState::new();

    println!("Done initializing, waiting for ESC startup...");
    // drivetrain_motors.startup();
    println!("ESC startup finished");
    // drivetrain::apply_inputs(&mut drivetrain_motors, 0.0, 0.25);
    // shooter.update(true);
    println!("asdf");

    //thread safe channel for Python -> Rust communication
    let (tx, rx) = mpsc::channel::<String>();

    // 1. Create a "Python Ready" flag
    let running = Arc::new(AtomicBool::new(true));
    let python_ready = Arc::new(AtomicBool::new(false));
    
    let thread_running = Arc::clone(&running);
    let main_ready_check = Arc::clone(&python_ready);
    
    // 2. Spawn the background thread for Python CV
    // We use a handle to join it later, ensuring it finishes cleanup.
    // Spawn the background thread for Python CV
    let python_thread_handle = thread::spawn(move || {
        Python::with_gil(|py| -> PyResult<()> {
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

            let module = PyModule::from_code(
                py, 
                py_code_c.as_c_str(), 
                file_name,
                module_name
            )?;
            
            println!("Rust: Creating Python ThreadHandler instance...");
            let handler_instance = module.getattr("access_python")?.call0()?;

            // --- HANDSHAKE: WAIT FOR SUB-THREADS ---
            println!("Rust: Waiting for Python sub-threads (Camera/YOLO) to report alive...");
            for _ in 0..150 { // Wait up to 15 seconds
                let alive: bool = handler_instance.call_method0("is_threads_running")?.extract()?;
                if alive {
                    python_ready.store(true, Ordering::SeqCst);
                    println!("Rust: Python CV Pipeline is fully ACTIVE.");
                    break;
                }
                // Drop GIL briefly to let Python threads work
                py.allow_threads(|| thread::sleep(Duration::from_millis(100)));
            }

            // --- POLLING LOOP ---
            while thread_running.load(Ordering::SeqCst) {
                let result_bound = handler_instance.call_method0("get_latest_results")?;
                
                if !result_bound.is_none() {
                    if let Ok(data) = result_bound.extract::<String>() {
                        let _ = tx.send(data);
                    }
                }

                // Drop GIL to allow Python sub-threads to process frames
                py.allow_threads(|| thread::sleep(Duration::from_millis(20)));
            }

            // --- SHUTDOWN ---
            println!("Rust: Signaling Python shutdown...");
            let _ = handler_instance.call_method0("stop");
            Ok(())
        }).expect("Python background thread panicked");
    });

    // 4. MAIN THREAD HANDSHAKE SYNC
    println!("Rust: Main thread waiting for CV handshake...");
    let startup_timer = Instant::now();
    while !main_ready_check.load(Ordering::SeqCst) {
        if startup_timer.elapsed() > Duration::from_secs(20) {
            println!("CRITICAL: Python initialization timeout. Exiting.");
            running.store(false, Ordering::SeqCst);
            return Ok(()); 
        }
        thread::sleep(Duration::from_millis(100));
    }

    // ... (Main Robot Loop remains the same)
    // Ensure you use rx.try_recv() to keep the robot loop fast!

    println!("entering Loop");
    
    //break out of main control loop with 'E' key
    let mut i: u32 = 0;
    loop {
        i += 1;
        // println!("i: {}", i);
        println!("Looping...");
        input.update();

        // Process all pending CV data
        while let Ok(cv_data) = rx.try_recv() {
            println!("Raw CV Data: {}", cv_data);
            
            // Parse the JSON string from Python
            let v: serde_json::Value = serde_json::from_str(&cv_data).unwrap_or(serde_json::Value::Null);
            
            if let Some(detections) = v.as_array() {
                for det in detections {
                    let x1 = det[0].as_f64().unwrap_or(0.0);
                    let conf = det[4].as_f64().unwrap_or(0.0);
                    println!("Target detected at X: {:.2} with confidence: {:.2}", x1, conf);
                    
                    // Example logic:
                    if x1 < 200.0 { println!("Steer Left!"); }
                    else if x1 > 400.0 { println!("Steer Right!"); }
                }
            }
        }

        // println!("asdf");
        // println!("x: {}", input.state.x);
        drivetrain::apply_inputs(&mut drivetrain_motors, input.state.left_stick_x * -1.0, input.state.left_stick_y, input.state.right_stick_x * -0.5);

        // shooter.update(true);
        shooter.update(input.state.y);
        intake.update(input.state.x);

        // let keys: Vec<Keycode> = device_state.get_keys();
        // if keys.contains(&Keycode::E) {
        //     running.store(false, Ordering::SeqCst); // Tell thread to die
        //     break;
        // }
        if poll_for_exit_key() {
            running.store(false, Ordering::SeqCst);
            break;
        }
        

        std::thread::sleep(Duration::from_millis(10));
    }

    // 6. CLEANUP & SHUTDOWN
    println!("Stopping hardware...");
    drivetrain::apply_inputs(&mut drivetrain_motors, 0.0, 0.0, 0.0);
    
    println!("Waiting for Python thread to exit (Max 10s)...");
    let shutdown_start = Instant::now();
    while !python_thread_handle.is_finished() {
        if shutdown_start.elapsed() > Duration::from_secs(10) {
            println!("Python thread timed out! Force exiting...");
            std::process::exit(1);
        }
        thread::sleep(Duration::from_millis(100));
    }

    let _ = python_thread_handle.join();
    println!("Shutdown complete.");
    crossterm::terminal::disable_raw_mode().unwrap();
    Ok(())
}

fn poll_for_exit_key() -> bool {
    if event::poll(Duration::from_millis(0)).unwrap_or(false) {
        if let Ok(Event::Key(key_event)) = event::read() {
            if key_event.code == KeyCode::Char('e') || key_event.code == KeyCode::Char('E') {
                return true;
            }
        }
    }
    false
}
