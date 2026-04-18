use std::io;
use std::net::{TcpListener, TcpStream};

pub struct Server {
    listener: TcpListener
}

impl Server {
    pub fn new() -> io::Result<Server> {
        let listener = TcpListener::bind("127.0.0.1:8080")?;
        Ok(Server {
            listener
        })
    }
    pub fn from(address: &str) -> io::Result<Server> {
        let listener = TcpListener::bind(address)?;
        Ok(Server {
            listener
        })
    }
}

impl IntoIterator for Server {
    type Item = TcpStream;
    type IntoIter = Box<dyn Iterator<Item = TcpStream>>;

    fn into_iter(self) -> Self::IntoIter {
        Box::new(self.listener.into_incoming().filter_map(Result::ok))
    }
}