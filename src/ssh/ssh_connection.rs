use crate::errors::SshError;
use crate::ssh::ssh_packet::SshPacket;
use std::io::Write;
use std::net::TcpStream;

pub const LOCALHOST_HOSTNAME: &str = "127.0.0.1";
pub const DEFAULT_TCP_PORT: &str = "22";

/// A `SshConnection` type to manage a connection to a host using ssh protocol
///
/// You have to connect to a host with a TCP port and an user, and next you could sand messages/commands.
///
/// # Examples
///
/// ```
/// let hostname = String::from(remconn::ssh::ssh_connection::LOCALHOST_HOSTNAME);
/// let tcp_port = String::from(remconn::ssh::ssh_connection::DEFAULT_TCP_PORT);
/// let user = String::from("admin");
///
/// let mut ssh_connection = SshConnection::new(hostname, tcp_port, user);
///
/// if let Err(e) = ssh_connection.connect() {
///     println!("Error : {0}", e);
///     return;
/// }
///
/// // You could do a loop
/// match ssh_connection.send(String::from("echo test")) {
///     Ok(_) => todo!(),
///     Err(e) => println!("Error : {0}", e),
/// }
///
/// ssh_connection.disconnect();
/// ```
#[derive(Debug)]
pub struct SshConnection {
    pub hostname: String,
    pub tcp_port: String,
    pub user: String,
    stream: Option<TcpStream>,
}

impl SshConnection {
    /// Create an instance
    ///
    /// Return a `SshConnection` instance if suceed, or a [`SshError`] otherwise
    ///
    /// [`SshError`]: SshError
    ///
    /// # Examples
    ///
    /// ```
    /// let hostname = String::from(remconn::ssh::ssh_connection::LOCALHOST_HOSTNAME);
    /// let tcp_port = String::from(remconn::ssh::ssh_connection::DEFAULT_TCP_PORT);
    /// let user = String::from("user");
    ///
    /// let mut ssh_connection = SshConnection::new(hostname, tcp_port, user);
    /// ```
    pub fn new(hostname: String, tcp_port: String, user: String) -> Self {
        Self {
            hostname,
            tcp_port,
            user,
            stream: None,
        }
    }

    /// Connect to the host
    ///
    /// Return nothing if succeed, or a [`SshError`] otherwise
    ///
    /// [`SshError`]: SshError
    ///
    /// # Examples
    ///
    /// ```no_run
    /// // ssh_connection already instanciate before
    /// if let Err(e) = ssh_connection.connect() {
    ///     println!("Error : {0}", e);
    /// }
    /// ```
    pub fn connect(&mut self) -> Result<(), SshError> {
        if let Some(_) = self.stream {
            return Err(SshError::new(String::from("Already connected")));
        }

        let host = format!("{0}:{1}", self.hostname, self.tcp_port);

        match TcpStream::connect(&host) {
            Ok(stream) => {
                println!("Should connect to {0}", host);
                self.stream = Some(stream);
                todo!()
            }
            Err(e) => Err(SshError::new(format!(
                "Couldn't connect to server : {0}",
                e,
            ))),
        }
    }

    /// Send message (or command) through the SSH connection
    ///
    /// Return nothing if succeed, or a [`SshError`] otherwise
    ///
    /// [`SshError`]: SshError
    ///
    /// # Examples
    ///
    /// ```no_run
    /// // ssh_connection already instanciate and been connected before
    /// if let Err(e) = ssh_connection.send(String::from("echo test")) {
    ///     println!("Error : {0}", e);
    /// }
    /// ```
    pub fn send(&mut self, command: String) -> Result<(), SshError> {
        match &mut self.stream {
            Some(stream) => {
                let ssh_packet = SshPacket::new(command);
                let buffer = ssh_packet.into_bytes();
                match stream.write(&buffer[..]) {
                    Ok(_) => {
                        // stream.read(&mut [0; 128]);
                        todo!()
                    }
                    Err(e) => {
                        return Err(SshError::new(format!(
                            "An error occured while sending datas : {0}",
                            e,
                        )))
                    }
                }
            }
            None => {
                return Err(SshError::new(String::from(
                    "You have to connect to the server before sending commands",
                )))
            }
        }
    }

    /// Disconnect from the SSH connection
    ///
    /// # Examples
    ///
    /// ```no_run
    /// // ssh_connection already instanciate and been connected before
    /// ssh_connection.disconnect();
    /// // Be care, instance is consume next this called
    /// ```
    pub fn disconnect(self) {
        println!(
            "Should disconnect from {0}:{1}",
            self.hostname, self.tcp_port
        );
        todo!();
    }
}
