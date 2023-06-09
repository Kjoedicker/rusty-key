use std::{
    fs::{File, OpenOptions},
    io::{Read, Write},
};

#[derive(Debug)]
pub struct FileHandler {
    file: File,
}

impl FileHandler {
    pub fn read_file(&mut self) -> String {
        let mut buffer = String::new();

        if let Err(error) = self.file.read_to_string(&mut buffer) {
            panic!("[ERROR] reading AOF: {:?}", error);
        };

        buffer
    }

    pub fn append_file(&mut self, buffer: &[u8]) {
        if let Err(error) = self.file.write(buffer) {
            panic!("[ERROR] appending to AOF: {:?}", error);
        };
    }

    pub fn parse_actions(&mut self) -> Vec<Vec<String>> {
        let file_string = self.read_file();

        let split_actions: Vec<&str> = file_string.split('\n').collect();

        split_actions
            .iter()
            .filter_map(|x| {
                let split: Vec<&str> = x.split('|').collect();
                let vector = split.iter().map(|x| x.to_string()).collect::<Vec<String>>();

                // When a newline character sneaks in 
                // this scenario becomes possible
                let is_not_empty = vector.len() > 1;
                if is_not_empty { Some(vector) } else { None }
            })
            .collect()
    }
}

pub fn init_file_handler(filepath: &'static str) -> FileHandler {
    let file_handler = OpenOptions::new()
        .append(true)
        .read(true)
        .write(true)
        .create(true)
        .open(filepath);

    let file = match file_handler {
        Ok(file) => file,
        Err(error) => panic!("[ERROR] creating file handler for AOF: {:?}", error),
    };

    FileHandler { file }
}
