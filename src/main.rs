use std::io;

use phaseshifter as ps;

fn main() {
    println!("Image batch converter. Type /h for help, /q to quit.");

    'main: loop {
        let mut command = String::new();
        io::stdin().read_line(&mut command).expect("Something broke.");

        command = command.trim().to_string();
  
        if command.eq("") {
            continue;
        } else if command.eq("/h") {
            println!("Usage:");
            println!("Single: <file path>");
            println!("Batch:  <directory path> -b");
            println!("Paths can be either relative or absolute.")
        } else if command.eq("/q") {
            break 'main;
        } else {
            match ps::process_input(&command) {
                Ok(_) => (),
                Err(e) => eprintln!("{}", e)
            };
        }
    }
}


