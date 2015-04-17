use std::io::prelude::*;
use std::net::TcpStream;
use std::str::from_utf8;

pub struct Player {
    pub stream: TcpStream
}

impl Player {
    pub fn new(stream: TcpStream) -> Player {
        return Player {
            stream: stream
        }
    }

    pub fn read(&mut self) {
        let mut buffer = [0u8; 512];
        loop {
            let usize = self.stream.read(&mut buffer).unwrap();
            if usize == 0 {
                break;
            }
            println!("{} {}", from_utf8(&buffer).unwrap(), usize);
        }
    }
}

