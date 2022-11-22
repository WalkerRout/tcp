#[derive(Debug)]
struct Client {
  stream: std::net::TcpStream
}

type ResBErr<T> = Result<T, Box<dyn std::error::Error + 'static>>;

impl Client {
  fn new(con: &str) -> ResBErr<Self> {
    let mut stream = std::net::TcpStream::connect(con);

    loop {
      match &stream {
        Ok(_) => break,
        Err(e) => {
          eprintln!("Error connecting to {}: {}", con, e);
          eprintln!("Retrying connection...");
          std::thread::sleep(std::time::Duration::from_millis(2500));
          stream = std::net::TcpStream::connect(con);
        }
      }
    }

    Ok(Client { stream: stream.unwrap() }) // error has been dealt with in an infinite loop, can only be broken if it is Ok(_)
  }

  fn send(&mut self, data: &[u8]) -> ResBErr<()> {
    use std::io::Write;

    match self.stream.write(data) {
      Ok(bytes_written) => {
        println!("Write success - Sent {:?} bytes!", bytes_written);
      },
      Err(e) => {
        eprintln!("Error writing to server: {}", e); 
        return Err(Box::new(e));
      }
    }

    Ok(())
  }
}

fn main() -> ResBErr<()> {
  println!("Client running...");
  let mut client = Client::new("127.0.0.1:34254")?;

  if let Some(arg) = std::env::args().nth(1) {
    println!("Sending: {}", &arg);
    client.send(arg.as_bytes())?;   
  } else {
    let s = format!("Connecting from {}\n", client.stream.local_addr()?);
    println!("Sending: {}", &s);
    client.send(s.as_bytes())?;
  }

  Ok(())
}
