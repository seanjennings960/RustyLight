extern crate weblights;

use std::env;
use weblights::ctrl::{Mode, lights};


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



fn main() {
    let mode = parse_input();
    match mode {
        Mode::On => println!("Turning on lights."),
        Mode::Off => println!("Turning off lights."),
    };
    lights(mode)
}
