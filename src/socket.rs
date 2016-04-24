use std::time::Duration;
use std::net::UdpSocket;
pub struct One<'r> { to_: &'r str, socket_: UdpSocket }

impl<'r> One<'r> {
    pub fn new(from: &'r str, to: &'r str) -> Self {
        let socket = UdpSocket::bind(from).unwrap();
        let _ = socket.set_read_timeout(Some(Duration::from_millis(1)));
        One {to_: to, socket_: socket}
    }
    pub fn recv(&self) -> Option<Vec<u8>> {
        let mut buf = [0; 15];
        let mut v = vec![];
        match self.socket_.recv_from(&mut buf) {
            Ok((amt,_)) => { v.extend_from_slice(&buf[..amt]); Some(v) }
            Err(_) => { None }
        }
    }
    pub fn send(&self, x: &Vec<u8>) {
        let _ = self.socket_.send_to(x, self.to_);
    }
}
