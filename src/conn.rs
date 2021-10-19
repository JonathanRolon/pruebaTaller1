//use crate::connections::Connections;

use std::{
    io::{Read, Write},
    net::TcpStream,
    sync::{Arc, Mutex},
};

#[derive(Clone)]
pub struct Conn {
    pub stream: Arc<Mutex<TcpStream>>
    //pub connections: Connections,
}

impl Conn {
    pub fn read(&self, mut buf: &mut [u8]) -> std::io::Result<usize> {
        self.stream.lock().unwrap().read(&mut buf)
    }

    pub fn write(&self, buf: &[u8]) -> std::io::Result<usize> {
        match self.stream.try_lock() {
            Ok(mut lock) => lock.write(buf),
            Err(_e) => Ok(0),
        }
    }

    pub fn take_error(&self) -> std::io::Result<Option<std::io::Error>> {
        self.stream.lock().unwrap().take_error()
    }
}
