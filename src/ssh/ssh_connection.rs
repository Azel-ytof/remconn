use crate::errors::SshError;
use crate::ssh::ssh_packet::SshPacket;
use std::io::Write;
use std::net::TcpStream;

pub const LOCALHOST_HOSTNAME: &str = "127.0.0.1";
pub const DEFAULT_TCP_PORT: &str = "22";

/// A `SshConnection` type to manage a connection to a host using ssh protocol
///
/// It is composed by a host name, a tcp port and an user.
///
/// # Examples
///
/// ```
/// let hostname = String::from(remconn::ssh::ssh_connection::LOCALHOST_HOSTNAME);
/// let tcp_port = String::from(remconn::ssh::ssh_connection::DEFAULT_TCP_PORT);
/// let user = String::from("admin");
///
/// match &mut SshConnection::connect(hostname, tcp_port, user) {
///     Ok(ssh_connection) => {
///         ssh_connection.send(String::from("echo test"));
///         ssh_connection.disconnect();
///     },
///     Err(e) => println!("Error : {0}", e)
/// }
/// ```
pub struct SshConnection {
    pub hostname: String,
    pub tcp_port: String,
    pub user: String,
    stream: TcpStream,
}

impl SshConnection {
    /// Create an instance by connecting to given host on given port with given user
    ///
    /// Return a `SshConnection` instance if suceed, or a [`SshError`] otherwise
    ///
    /// [`SshError`]: SshError
    pub fn connect(hostname: String, tcp_port: String, user: String) -> Result<Self, SshError> {
        let host = format!("{0}:{1}", hostname, tcp_port);

        match TcpStream::connect(&host) {
            Ok(stream) => {
                println!("Should connect to {0}", host);
                Ok(Self {
                    hostname,
                    tcp_port,
                    user,
                    stream,
                })
            }
            Err(e) => Err(SshError::new(format!(
                "Couldn't connect to server : {0}",
                e,
            ))),
        }
    }

    /// Send message (or command) through the SSH connection
    pub fn send(&mut self, command: String) {
        let ssh_packet = SshPacket::new(command);
        let buffer = ssh_packet.into_bytes();

        match self.stream.write(&buffer[..]) {
            Ok(_) => println!("ok"),
            Err(e) => println!("err : {0}", e),
        }
        // stream.read(&mut [0; 128]);
        todo!()
    }

    /// Disconnect from the SSH connection
    pub fn disconnect(self) {
        println!(
            "Should disconnect from {0}:{1}",
            self.hostname, self.tcp_port
        );
        todo!();
    }
}
