//! # SshConnection
//!
//! TODO
//!

use crate::ssh::ssh_packet::SshPacket;
use std::io::Write;
use std::net::TcpStream;

pub struct SshConnection {
    pub hostname: String,
    pub tcp_port: String,
    pub user: String,
    stream: Option<TcpStream>,
}

impl SshConnection {
    pub fn new(hostname: String, tcp_port: String, user: String) -> Self {
        Self {
            hostname,
            tcp_port,
            user,
            ..Default::default()
        }
    }

    pub fn connect(&mut self) {
        match self.stream {
            Some(_) => println!("Already connected"),
            None => {
                let host = format!("{0}:{1}", self.hostname, self.tcp_port);

                if let Ok(stream) = TcpStream::connect(&host) {
                    println!("Should connect to {0}", host);
                    self.stream = Some(stream);
                } else {
                    println!("Couldn't connect to server...");
                }
            }
        }
    }

    pub fn write(&mut self, command: String) {
        match &mut self.stream {
            Some(stream) => {
                let ssh_packet = SshPacket::new(command);
                stream.write(ssh_packet.into_bytes());
                // stream.read(&mut [0; 128]);
                todo!()
            }
            None => println!("You have to connect to the server before sending commands"),
        }
    }

    pub fn disconnect(&mut self) {
        match &self.stream {
            Some(_) => {
                println!(
                    "Should disconnect from {0}:{1}",
                    self.hostname, self.tcp_port
                );
                self.stream = None;
            }
            None => println!("Already disconnected"),
        }
    }
}

impl Default for SshConnection {
    fn default() -> Self {
        Self {
            hostname: String::from("127.0.0.1"),
            tcp_port: String::from("22"),
            user: String::from("user"),
            stream: None,
        }
    }
}
