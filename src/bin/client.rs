
use std::fmt::write;

use tokio::{net::TcpStream, io::{BufReader, AsyncBufReadExt, AsyncWriteExt}};

#[tokio::main]
async fn main(){
     let mut stream = TcpStream::connect("127.0.0.1:8000").await.unwrap();
     let (mut reader,mut writer) = stream.into_split();

     let read_handle = tokio::spawn(async move{
          let mut reader = BufReader::new(reader);
          let mut line = String::new();
          loop{
               match reader.read_line(&mut line).await {
                    Ok(0) => {
                        println!("Server disconnected");
                        break;
                    }
                    Ok(_) => {
                        println!("{}", line.trim());
                        line.clear();
                    }
                    Err(err) => {
                        eprintln!("Error reading from server: {}", err);
                        break;
                    }
                }
          }
     });

     let write_handle = tokio::spawn(async move{
          let mut message = String::new();
          loop {
               std::io::stdin().read_line(&mut message).expect("Failed to read input");
               writer.write_all(message.as_bytes()).await.unwrap();
          }   
     });

     read_handle.await.unwrap();
     write_handle.await.unwrap();
}