use std::{fs::{OpenOptions, File}, io::{Read, Write}};

struct FileHandler {
    filepath:  & 'static str,
    file: File
}

impl FileHandler {
    fn read_file(&mut self) -> String {
        let mut buffer = String::new();
    
        let file_string = self.file.read_to_string(&mut buffer);

        match file_string {
            Ok(file) => file,
            Err(err) => panic!("Error reading file{:?}", err)
        };

        buffer
    }

    fn append_file(&mut self, buffer: &[u8]) {
        match self.file.write(buffer) {
            Err(err) => panic!("Error writing to AOF: {:?}", err),
            _ => {}
        };
    }

    fn delete_file() {
        // TODO: implement me
    }
}

fn init_file_handler(filepath: & 'static str) -> FileHandler {

    let file_handler = OpenOptions::new()
        .append(true)
        .read(true)
        .write(true)
        .create(true)
        .open(filepath);

    let file = match file_handler {
        Ok(file) => file,
        _ => panic!("Error creating file handler for AOF")
    };

    FileHandler { 
        filepath,
        file
    }
}