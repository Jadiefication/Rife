use std::io;
use std::net::TcpListener;

pub struct Server {
    listener: TcpListener
}

impl Server {
    fn new() -> io::Result<Server> {
        let listener = TcpListener::bind("127.0.0.1:80")?;
        Ok(Server {
            listener
        })
    }
    fn from(address: &str) -> io::Result<Server> {
        let listener = TcpListener::bind(address)?;
        Ok(Server {
            listener
        })
    }
}

#[macro_export]
macro_rules! server {
    () => {
        Server::new()
    };
    ($address:expr) => {
        Server::from($address)
    }
}