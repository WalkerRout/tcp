#[derive(Debug)]
struct Server {
  listener: std::net::TcpListener
}

type ResBErr<T> = Result<T, Box<dyn std::error::Error + 'static>>;

impl Server {
  fn new(con: &str) -> ResBErr<Self> {
    let listener = std::net::TcpListener::bind(con)?;
    Ok(Server { listener })
  }

  fn listen(&mut self, handle_connection_fn: impl Fn(std::net::TcpStream) -> ResBErr<()>) -> ResBErr<()> {

    for stream in self.listener.incoming() {
      match stream {
        Ok(stream) => {
          println!("New client from: {:?}", stream.peer_addr()?);
          handle_connection_fn(stream)?;
        },
        Err(e) => { 
          eprintln!("Error in accepting: {}", e); 
        }
      }
    }

    Ok(())
  }
}
