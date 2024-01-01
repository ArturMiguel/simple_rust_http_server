use server::Server;
use http::Method;
use http::Request;

mod http;
mod server;

fn main() {
  let server: Server = server::Server::new(String::from("127.0.0.1:8080"));
  server.run();
}




