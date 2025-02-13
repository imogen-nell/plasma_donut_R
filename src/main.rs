use std::time::Duration;
use simple_logger::SimpleLogger;

use plasma_donut::controller::ControllerThread;

use plasma_donut::cmd;

const DIR : i32  = 18;
const PUL : i32  = 17;

fn main() {

    // Create a new ScienceSetUp instance
    let setup = ScienceSetUp::new();

    //initialize the stepper controller thread
    let controller = setup.stepper_ctrlr;

    // Initialize the logger
    SimpleLogger::new().init().unwrap();

    loop{
        // example 
        // Send a command to the controller
        controller.inner.try_send(cmd::SetDirection(DIR));
        // Wait for 1 second
        std::thread::sleep(Duration::from_secs(1));
        // Send a command to the controller
        controller.inner.try_send(cmd::DisableMotor());
        // Wait for 1 second
        println!("before : {:?}", my_controller.inner.try_iter().last());
        sleep(Duration::from_millis(3000));
        println!("after  : {:?}", my_controller.inner.try_iter().last());
    }
}
