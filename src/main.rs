use std::{
    collections::HashMap,
    fs,
    io::{BufRead, BufReader},
    process::{Command, Stdio},
};

use glob::glob;

const KEYBOARD_LED_PATTERN: &str = "/sys/class/leds/*::kbd_backlight/brightness";

fn main() {
    let command = Command::new("acpi_listen")
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to spawn acpi_listen");
    let mut output = BufReader::new(command.stdout.unwrap());
    // FIXME: Handle booting in tablet mode
    let mut tablet_mode = false;
    let mut saved_brightnesses = HashMap::new();
    loop {
        let mut line = String::new();
        output.read_line(&mut line).unwrap();
        let data: Vec<_> = line.split_whitespace().collect();
        if data[0] == "video/tabletmode" {
            let tablet_mode_event = match data[3] {
                "00000001" => Some(true),
                "00000000" => Some(false),
                _ => None,
            };
            match tablet_mode_event {
                Some(on) => {
                    if on != tablet_mode {
                        match on {
                            true => {
                                println!("Entered tablet mode");
                                let glob = glob(KEYBOARD_LED_PATTERN);
                                match glob {
                                    Ok(glob) => {
                                        for led in glob {
                                            match led {
                                                Ok(led) => match fs::read_to_string(&led) {
                                                    Ok(brightness) => {
                                                        match brightness.trim().parse::<u32>() {
                                                            Ok(brightness) => {
                                                                saved_brightnesses.insert(
                                                                    led.clone(),
                                                                    brightness,
                                                                );
                                                                match fs::write(&led, 0.to_string())
                                                                {
                                                                    Ok(_) => {}
                                                                    Err(e) => {
                                                                        println!("Failed to set brightness to 0 for {:?}: {:#?}", led, e)
                                                                    }
                                                                }
                                                            }
                                                            Err(e) => {
                                                                println!("Failed to parse brightness for {:?}: {:#?}", led, e)
                                                            }
                                                        }
                                                    }
                                                    Err(e) => {
                                                        println!("Failed to read brightness for {:?}: {:#?}",  led, e);
                                                    }
                                                },
                                                Err(e) => {
                                                    println!("Failed to glob brightness: {:#?}", e)
                                                }
                                            }
                                        }
                                    }
                                    Err(e) => {
                                        println!("Failed to glob brightness: {:#?}", e)
                                    }
                                }
                                println!("Saved brightnesses: {:#?}", saved_brightnesses);
                            }
                            false => {
                                println!("Exited tablet mode");
                                let glob = glob(KEYBOARD_LED_PATTERN);
                                match glob {
                                    Ok(glob) => {
                                        for led in glob {
                                            match led {
                                                Ok(led) => match saved_brightnesses.get(&led) {
                                                    Some(brightness) => {
                                                        match fs::write(
                                                            &led,
                                                            brightness.to_string(),
                                                        ) {
                                                            Ok(_) => {}
                                                            Err(e) => {
                                                                println!("Failed to restore brightness for {:?}: {:#?}",  led, e);
                                                            }
                                                        }
                                                    }
                                                    None => {
                                                        println!(
                                                            "No saved brightness for {:?}",
                                                            led
                                                        );
                                                    }
                                                },
                                                Err(e) => {
                                                    println!("Failed to glob brightness: {:#?}", e)
                                                }
                                            }
                                        }
                                    }
                                    Err(e) => {
                                        println!("Failed to glob brightness: {:#?}", e)
                                    }
                                }
                                saved_brightnesses.clear();
                            }
                        }
                        tablet_mode = on
                    }
                }
                None => println!("Tablet mode unknown mode"),
            }
        }
    }
}
