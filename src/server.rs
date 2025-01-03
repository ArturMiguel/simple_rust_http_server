use std::{net::TcpListener, io::Read};

pub struct Server {
  addr: String
}

impl Server {
  pub fn new(addr: String) -> Self {
    Self {
      addr
    }
  }

  pub fn run(self) {
    let listener = TcpListener::bind(&self.addr).unwrap();

    println!("Listening on {}", self.addr);
    
    loop {
      match listener.accept() {
        Ok((mut stream, _addr)) => {
          let mut buffer = [0; 1024];

          match stream.read(&mut buffer) {
            Ok(_) => {
              println!("Received a request: {}", String::from_utf8_lossy(&buffer))
            },
            Err(e) => println!("Failed to read from connection: {:?}", e)
          }
        },
        Err(e) => println!("Failed to establish a connection: {:?}", e),
      }
    }
  }
}