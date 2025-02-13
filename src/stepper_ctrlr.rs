use std::time::Duration;
use std::thread::sleep;
use std::thread;
use std::error::Error;
use std::io::{self, Write};
use cadh::threadsync::DualChannelSync;
use crossbeam;
use log::{info, warn, error};


extern crate rppal; 
use rppal::gpio::{Gpio, OutputPin};
use std::;


const SLEEP : f64 = 0.005;//in seconds, 5ms

type Result<T> = std::result::Result<T, Box<dyn Error>>;

pub struct ControllerThread {
    pub inner: DualChannelSync<cmd, data>,

}

pub struct Controller{
    DIR : OutputPin, // Direction pin 
    PUL : OutputPin, // Pulse pin
    step_delay : f32, // Delay between steps in ms
    dirrection : i32, // Direction of the motor(0 = clockwise, 1 = counter-clockwise)

}

#[derive(Debug)]
#[derive(PartialEq)]
pub enum readstate{
    Read,
}

pub enum cmd{
    SetDirection(i32),
    DisableMotor(),
}

#[derive(Debug)]
pub struct data{
    pub feedback: f32,
}

impl ControllerThread {

    pub fn spawn_feedback_thread(frequency: u32) -> Result<Self> {

        let handle = DualChannelSync::spawn(

            "Feedback Updater",
            
            move |to_main: _, from_main: _| {
            
                log::info!("Feedback thread started!");
                //initialize controller, give ownership to the thread
                let mut controller = Controller::new(frequency).expect("Failed to init controller");
                //initialize variables
                let step_delay = 1.0 / frequency;
                let dirrection = 0; // 0 = clockwise, 1 = counter-clockwise
        
                //init gpio interface
                let gpio = Gpio::new().expect("Failed to access GPIO");
        
                //init pins
                let pul = gpio.get(controller::).expect("Failed to access PUL pin").into_output();
                let dir = gpio.get(DIR_PIN).expect("Failed to access DIR pin").into_output();
        
                //set initial dirrection
                dir_pin.write(if dir == 0 { rppal::gpio::Level::Low } else { rppal::gpio::Level::High });
        
                //setup timing
                let interval = Duration::from_secs_f64(SLEEP);
                let mut next_update = Instant::now() + interval;

                loop {
                    //check for commands from main, if any then excecute
                    if from_main.len() > 10 {log::warn!("{} commands in queue", from_main.len());}
                    for cmd in from_main.try_iter() { 
                        //execute command                       
                        match cmd {
                            cmd::SetDirection(dir) => { ControllerThread::set_dirrection(&mut controller, current);}
                            cmd::DisableMotor() => { ControllerThread::disable_motor();}
                        }
                        //wait appropriate time
                        if (std::time::Instant::now())  > next_update {next_update = Instant::now() + interval ;}
                        sleep(next_update-Instant::now());
                        next_update += interval;

                    }

                    // TODO : read feedback   
                    //populate data struct
                    // log::debug!("state: {:?}", controller.state);
                    // let data = data { feedback: controller.feedback};
                    //send data to main
                    // if let Err(e) = to_main.send(data) {
                    //     log::warn!("failed to send data: {}", e);
                    // }        
                                
                }
            })?;

            Ok(ControllerThread {
                inner: handle,
            })
            
    }

    fn set_dirrection(controller: &mut Controller, current: f32) {
        //TODO fix tthis 
        controller.port.write(&Controller::create_command("current", current));
    }

    fn disable_motor() {
        //TODO fix tthis 
        controller.port.write(&Controller::create_command("current", current));
    }

}

impl Controller {

    //creates a controller
    pub fn new( frequency: u32) -> Result<Self> {


        Ok( Controller {
            feedback: 0.0,
            state: readstate::Lost,
            // status: [0; 10],
        })
    }
     
    //calculates the checksum for the given packet
    //Arguments: cmd: &[u8] - the byte packet including the header, may include the crc or not 
    //Returns: u16 - the calculated checksum in the order it should appear in the command send 
    // fn checksum(cmd: &[u8]) -> u16 {
    //     let msg = &cmd[2..10];
    //     let mut crc16: u16 = 0xFFFF; // initial value FFFF always
    //     for &byte in msg{
    //         let i = ((crc16>>8)^ byte as u16) as usize;
    //         crc16 = CRC16_TABLE[i] ^ (crc16 << 8);
    //     }
    //     crc16.swap_bytes()
    // }
}