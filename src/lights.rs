extern crate sysfs_gpio;

use std::env;
use sysfs_gpio::{Direction, Pin};
use std::thread::sleep;
use std::time::Duration;

const ON_PIN: u64 = 23;
const OFF_PIN : u64 = 60;
const SLEEP_TIME: u64 = 500;

enum Mode {
    On,
    Off,
}

fn parse_input() -> Mode {
    let args: Vec<String> = env::args().collect();

    let len = args.len();
    if len != 2 {
        panic!("Expected 1 command line arg not {:}", len);
    }
    match args[1].as_str() {
        "on" => Mode::On,
        "off" => Mode::Off,
        _ => panic!("Mode must either be 'on' or 'off'"),
    }
    // println!("Command line arg: {}", len);
    // Mode::On
}

fn lights(mode: Mode) {
    let pin_num = match mode {
        Mode::On => ON_PIN,
        Mode::Off => OFF_PIN,
    };
    let pin = Pin::new(pin_num);
    println!("PIN: {:?}", pin);
    let result = pin.with_exported(|| {
        pin.set_direction(Direction::Out).unwrap();
        pin.set_value(1)?;
        sleep(Duration::from_millis(SLEEP_TIME));
        pin.set_value(0)?;
        println!("Successfully set back to 0.");
        Ok(())
    });
    match result {
        Ok(_) => (),
        Err(e) => println!("Error using pin: {:?}", e),
    };
}


fn main() {
    let mode = parse_input();
    match mode {
        Mode::On => println!("Turning on lights."),
        Mode::Off => println!("Turning off lights."),
    };
    lights(mode)
}
