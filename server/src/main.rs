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

fn handle_connection(mut stream: std::net::TcpStream) -> ResBErr<()> {
    use std::io::Read;

    let mut buffer = vec![0; 256];
    stream.read(&mut buffer)?;

    if compare(&buffer[..], &[80, 65, 78, 73, 67]) == std::cmp::Ordering::Equal {
      panic!("'PANIC' SENT!");
    }
    
    let buffer_string = String::from_utf8_lossy(&buffer);

    println!("-- Data from connection --
           \nBytes:{:?}
           \nString Representation: {}\n", 
           &buffer[..], &buffer_string);

    Ok(())
  }

fn compare(a: &[u8], b: &[u8]) -> std::cmp::Ordering {
  for (ai, bi) in a.iter().zip(b.iter()) {
    match ai.cmp(&bi) {
      std::cmp::Ordering::Equal => continue,
      ord => return ord
    }
  }

  std::cmp::Ordering::Equal
}
  
fn main() -> ResBErr<()> {
  let ipp = "127.0.0.1:34254";

  println!("Server Running on {} ...", ipp);

  let mut server = Server::new(ipp)?;
  server.listen(|stream| handle_connection(stream))?;
  
  Ok(())
}
