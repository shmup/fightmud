use std::net::{TcpListener, TcpStream};
use std::thread;
use std::io::prelude::*;
use std::io::{BufWriter, BufReader};

struct Server {
    pub ip: &'static str,
    pub port: i32,
}

impl Server {
    pub fn start(&self) {
        let listener = TcpListener::bind("127.0.0.1:1337").unwrap();

        for stream in listener.incoming() {
            match stream {
                Ok(mut stream) => {
                    handle_client(stream);
                }
                Err(e) => { println!("Connection failed"); }
            }
        }

        drop(listener);
    }
}

fn handle_client(mut stream: TcpStream) {}

pub struct ServerFactory {
    pub ip: &'static str,
    pub port: i32,
}

impl ServerFactory {
    pub fn new() -> ServerFactory {
        ServerFactory { ip: "127.0.0.1", port: 80 }
    }

    pub fn ip(&mut self, ip: &'static str) -> &mut ServerFactory {
        self.ip = ip;
        self
    }

    pub fn port(&mut self, port: i32) -> &mut ServerFactory {
        self.port = port;
        self
    }

    pub fn create(&self) -> Server {
        Server { ip: self.ip, port: self.port }
    }
}

#[test]
fn custom_port_works() {
    let server = ServerFactory::new() 
        .port(1337)
        .create();

    assert_eq!(server.port, 1337);
}

#[test]
fn custom_ip_works() {
    let server = ServerFactory::new() 
        .ip("1.3.3.7")
        .create();

    assert_eq!(server.ip, "1.3.3.7");
}
