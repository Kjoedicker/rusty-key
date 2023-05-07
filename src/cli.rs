use crate::kv_store::KEY_VALUE_STORE;
use regex::{Captures, Regex};
use std::io;

fn capture_command_arguments(input_string: &String) -> Option<Captures> {
    let re = Regex::new(r"(?m)^(?P<command>.+?)\s+(?P<key>.+?)\s+(?P<value>.+?)?$").unwrap();

    let captured_arguments = re.captures(input_string);

    captured_arguments
}

fn process_captured_arguments(captured_arguments: Captures) {
    let matched_command = captured_arguments.name("command").unwrap();

    let mut teller = KEY_VALUE_STORE.lock().unwrap();

    match matched_command.as_str() {
        "set" => {
            let key = captured_arguments.name("key").unwrap().as_str().to_owned();
            let value = captured_arguments
                .name("value")
                .unwrap()
                .as_str()
                .to_owned();

            teller.set(key.clone(), value.clone(), Some(true));

            println!("[SUCCESS] Key {} added with a value of {}", key, value);
        }
        "get" => {
            let key = captured_arguments.name("key").unwrap().as_str();

            match teller.get(key) {
                Some(value) => println!("[SUCCESS] {value}"),
                None => println!("[FAILURE] Key does not exist: {}", key),
            };
        }
        "delete" => {
            let key = captured_arguments.name("key").unwrap().as_str();

            match teller.delete(key, Some(true)) {
                Some(value) => println!("{value}"),
                None => {
                    println!("[FAILURE] Key does not exist: {}", key);
                }
            };
        }
        _ => {
            println!("[FAILURE] {} is not a valid command", matched_command.as_str())
        }
    }
}

pub fn start() {
    println!("Rusty-Key CLI Beta");

    loop {
        let mut input_string = String::new();

        io::stdin().read_line(&mut input_string).unwrap();

        let exiting_gracefully = input_string.trim() == "exit";
        if exiting_gracefully {
            println!("[EXITING]");
            break;
        };

        let command_match = capture_command_arguments(&input_string);

        match command_match {
            Some(captures) => {
                process_captured_arguments(captures);
            }
            None => {
                println!("[FAILURE] invalid command")
            }
        }
    }
}
