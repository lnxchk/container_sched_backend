extern crate toml;

use toml::Value;
use std::fs::File;
use std::io::prelude::*;
use std::{thread, time};

fn main() {
    let mut file = File::open("config/config.toml").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
	println!("{}", contents);
    let value = contents.parse::<Value>().unwrap();
//	println!("After Parse into Value");
//	println!("{}", value);
    let five_seconds = time::Duration::new(5, 0);

    loop {
//	println!("In Loop");
        if value["myconfig"]["leader"].as_bool().unwrap() {
            println!("I am a leader");
        } else {
            println!("I am a follower");
        }
	let color = value["myconfig"]["color"].as_str().unwrap();
	println!("My favorite color is {}", color);

        thread::sleep(five_seconds);
    }
}
