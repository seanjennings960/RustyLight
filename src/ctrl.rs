extern crate sysfs_gpio;

use std::fmt;
use std::str::FromStr;
use sysfs_gpio::{Direction, Pin};
use std::thread::sleep;
use std::time::Duration;


const ON_PIN: u64 = 23;
const OFF_PIN : u64 = 60;
const SLEEP_TIME: u64 = 500;

pub enum Mode {
    On,
    Off,
}

pub struct ParseModeErr {
    input: String
}

impl fmt::Display for ParseModeErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Mode must be on or off, not {}", self.input)
    }
}

impl FromStr for Mode {
    type Err = ParseModeErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "on" => Ok(Mode::On),
            "off" => Ok(Mode::Off),
            _ => Err(ParseModeErr{input: s.to_string()}),
        }
    }
}

pub fn lights(mode: Mode) {
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
