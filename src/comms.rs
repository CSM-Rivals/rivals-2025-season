use std::net::{TcpListener, TcpStream};  
use std::sync::{Arc, Mutex};
use std::thread;
use serde::{Serialize, Deserialize};
use rmp_serde;

pub struct Input {
    listener: TcpListener,

    pub state: Packet,

    last_packet: Arc<Mutex<Packet>>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Copy)]
pub struct Packet {
    pub left_stick_x: f64,
    pub left_stick_y: f64,
    pub right_stick_x: f64,
    pub right_stick_y: f64,

    pub a: bool,
    pub b: bool,
    pub x: bool,
    pub y: bool,
}

impl Packet {
    pub fn new() -> Packet {
        Packet { left_stick_x: 0.0, left_stick_y: 0.0, right_stick_x: 0.0, right_stick_y: 0.0, a: false, b: false, x: false, y: false }
    }
}

impl Input {
    pub fn new() -> Input {
        let listener = TcpListener::bind("0.0.0.0:29230").unwrap();
        Input {
            listener,

            state: Packet::new(),

            last_packet: Arc::new(Mutex::new(Packet::new())),
        }
    }

    fn handle_connection(stream: TcpStream, last_packet: Arc<Mutex<Packet>>) {
        loop {
            match rmp_serde::from_read::<&TcpStream, Packet>(&stream) {
                Ok(packet) => {
                    *last_packet.lock().unwrap() = packet;
                },
                Err(_) => {
                    println!("Packet read failed");
                    break;
                },
            }
        }
        println!("Disconnect");
    }

    pub fn update(&mut self) {
        {
            let last_packet = self.last_packet.lock().unwrap();

            self.state = last_packet.clone();
        }

        for stream in self.listener.incoming() {
            match stream {
                Ok(stream) => {
                    let arc = self.last_packet.clone();
                    thread::spawn(move || {
                        Input::handle_connection(stream, arc);
                    });
                },
                Err(_) => {},
            }
        }
    }
}