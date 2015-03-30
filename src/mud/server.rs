use std::net::{TcpStream, TcpListener};
use std::thread;

// struct Server {
//     ip: &str,
//     port: i32
// }

// impl Server {
//     pub fn new() {
//         let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

//         for stream in listener.incoming() {
//             match stream {
//                 Ok(stream) => {
//                     println!("Someone conencted!");
//                 }
//                 Err(e) => {
//                     println!("There was an error!")
//                 }
//             }
//         }
//     }
//     fn handle_client(mut stream: TcpStream) {
//     }
// }

struct Server {
    ip: String,
    port: i32,
}

pub struct ServerFactory {
    pub ip: &'static str,
    pub port: i32,
}

impl ServerFactory {
    pub fn new() -> ServerFactory {
        ServerFactory { ip: "127.0.0.1", port: 80 }
    }

    pub fn ip(&mut self, ip: &str) -> &mut ServerFactory {
        self.ip = ip;
        self
    }

    pub fn port(&mut self, port: i32) -> &mut ServerFactory {
        self.port = port;
        self
    }

    fn start(&self) -> Server {
        Server { ip: self.ip, port: self.port }
    }
}

