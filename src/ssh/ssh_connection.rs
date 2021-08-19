use crate::errors::SshError;
use crate::ssh::ssh_packet::SshPacket;
use std::io::{Read, Write};
use std::net::{Shutdown, TcpStream};

pub const LOCALHOST_HOSTNAME: &str = "127.0.0.1";
pub const DEFAULT_TCP_PORT: &str = "22";

/// A `SshConnection` type to manage a connection to a host using ssh protocol
///
/// You have to connect to a host with a TCP port and an user, and next you could sand messages/commands.
///
/// # Examples
///
/// ```
/// use remconn::ssh::ssh_connection::SshConnection;
///
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
/// if let Err(e) = ssh_connection.send(String::from("echo test")) {
///     println!("Error : {0}", e);
/// }
///
/// match ssh_connection.read() {
///     Ok(out) => println!("Received {0}", out),
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
    /// use remconn::ssh::ssh_connection::SshConnection;
    ///
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
    /// ```
    /// use remconn::ssh::ssh_connection::SshConnection;
    ///
    /// let hostname = String::from(remconn::ssh::ssh_connection::LOCALHOST_HOSTNAME);
    /// let tcp_port = String::from(remconn::ssh::ssh_connection::DEFAULT_TCP_PORT);
    /// let user = String::from("user");
    ///
    /// let mut ssh_connection = SshConnection::new(hostname, tcp_port, user);
    ///
    /// // ssh_connection already instanciate before
    /// if let Ok(_) = ssh_connection.connect() {
    ///     println!("Connected to the server !");
    /// } else {
    ///     println!("Couldn't connect to the server...");
    /// }
    /// ```
    pub fn connect(&mut self) -> Result<(), SshError> {
        if let Some(_) = self.stream {
            return Err(SshError::new(String::from("Already connected")));
        }

        let host = format!("{0}:{1}", self.hostname, self.tcp_port);

        match TcpStream::connect(&host) {
            Ok(stream) => {
                self.stream = Some(stream);
                Ok(())
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
    /// ```
    /// use remconn::ssh::ssh_connection::SshConnection;
    ///
    /// let hostname = String::from(remconn::ssh::ssh_connection::LOCALHOST_HOSTNAME);
    /// let tcp_port = String::from(remconn::ssh::ssh_connection::DEFAULT_TCP_PORT);
    /// let user = String::from("user");
    ///
    /// let mut ssh_connection = SshConnection::new(hostname, tcp_port, user);
    ///
    /// // Result to handle
    /// ssh_connection.connect();
    ///
    /// if let Ok(_) = ssh_connection.send(String::from("echo test")) {
    ///     println!("Message sent to the host");
    /// } else {
    ///     println!("Couldn't send message to the host");
    /// }
    /// ```
    pub fn send(&mut self, command: String) -> Result<(), SshError> {
        match &mut self.stream {
            Some(stream) => {
                let ssh_packet = SshPacket::new(command);

                match ssh_packet.into_bytes() {
                    Ok(ssh_packet_bytes) => {
                        if let Err(e) = stream.write(&ssh_packet_bytes[..]) {
                            return Err(SshError::new(format!(
                                "An error occured while sending datas : {0}",
                                e,
                            )));
                        }

                        Ok(())
                    }
                    Err(e) => Err(SshError::new(format!(
                        "An error occured while creating SSH packets : {0}",
                        e
                    ))),
                }
            }
            None => {
                return Err(SshError::new(String::from(
                    "You have to connect to the server before sending commands",
                )))
            }
        }
    }

    /// Read message through the SSH connection
    ///
    /// Return the output if succeed, or a [`SshError`] otherwise
    ///
    /// [`SshError`]: SshError
    ///
    /// # Examples
    ///
    /// ```
    /// use remconn::ssh::ssh_connection::SshConnection;
    ///
    /// let hostname = String::from(remconn::ssh::ssh_connection::LOCALHOST_HOSTNAME);
    /// let tcp_port = String::from(remconn::ssh::ssh_connection::DEFAULT_TCP_PORT);
    /// let user = String::from("user");
    ///
    /// let mut ssh_connection = SshConnection::new(hostname, tcp_port, user);
    ///
    /// // Result to handle
    /// ssh_connection.connect();
    ///
    /// if let Ok(out) = ssh_connection.read() {
    ///     println!("Message received from the host : {0}", out);
    /// } else {
    ///     println!("Couldn't receive message from the host");
    /// }
    /// ```
    pub fn read(&mut self) -> Result<String, SshError> {
        let buffer = &mut [0; 128];

        match &mut self.stream {
            Some(stream) => {
                if let Err(e) = stream.read(buffer) {
                    return Err(SshError::new(format!(
                        "An error occured while reading datas : {0}",
                        e,
                    )));
                }

                Ok(self.get_string_from_slice(buffer))
            }
            None => {
                return Err(SshError::new(String::from(
                    "You have to connect to the server before reading",
                )))
            }
        }
    }

    /// Convert slice of byte to String
    /// Non UTF-8 character will be replace by '?'
    fn get_string_from_slice(&self, slice: &[u8]) -> String {
        match String::from_utf8(slice.to_vec()) {
            Ok(t) => t,
            Err(_) => {
                let t = String::from_utf8_lossy(slice);
                t.to_string()
            }
        }
    }

    /// Disconnect from the SSH connection
    ///
    /// # Examples
    ///
    /// ```
    /// use remconn::ssh::ssh_connection::SshConnection;
    ///
    /// let hostname = String::from(remconn::ssh::ssh_connection::LOCALHOST_HOSTNAME);
    /// let tcp_port = String::from(remconn::ssh::ssh_connection::DEFAULT_TCP_PORT);
    /// let user = String::from("user");
    ///
    /// let mut ssh_connection = SshConnection::new(hostname, tcp_port, user);
    ///
    /// // Result to handle
    /// ssh_connection.connect();
    ///
    /// if let Err(e) = ssh_connection.disconnect() {
    ///     println!("Error : {0}", e);
    /// }
    /// // Be care, the instance is consume next this called
    /// ```
    pub fn disconnect(self) -> Result<(), SshError> {
        match self.stream {
            Some(stream) => {
                if let Err(e) = stream.shutdown(Shutdown::Both) {
                    Err(SshError::new(format!(
                        "An error occured while shutting down the stream : {0}",
                        e,
                    )))
                } else {
                    Ok(())
                }
            }
            None => Err(SshError::new(String::from(
                "You have to connect to the server before disconnecting",
            ))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_connect_unknown_host() {
        let hostname = String::from("unknown_host");
        let tcp_port = String::from(DEFAULT_TCP_PORT);
        let user = String::from("admin");

        let mut ssh_connection = SshConnection::new(hostname, tcp_port, user);

        assert!(ssh_connection.connect().is_err());
    }
}
