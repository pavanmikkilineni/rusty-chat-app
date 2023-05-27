//This code creates a TCP server, 
//accepts client connections, 
//and relays messages between clients using a broadcast channel. 
//Each client connection is handled concurrently in its own task using tokio::spawn.

use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpListener,
    sync::broadcast,
};

#[tokio::main]
async fn main() {

    let listener = TcpListener::bind("127.0.0.1:8000").await.unwrap();
    //broadcat channel - multiple producer, mutliple consumer.
    let (tx, _rx) = broadcast::channel(10);
    loop {
        let (mut socket, addr) = listener.accept().await.unwrap();
        let tx = tx.clone();
        let mut rx = tx.subscribe();
        tokio::spawn(async move {
            let (reader, mut writer) = socket.split();
            let mut reader = BufReader::new(reader);
            let mut line = String::new();
            loop {
                //Waits on multiple concurrent branches, returning when the first branch completes, cancelling the remaining branches.
                tokio::select! {
                    result = reader.read_line(&mut line) => {
                          if result.unwrap() == 0{
                            break;
                          }
                          tx.send((line.clone(),addr)).unwrap();
                          line.clear(); 
                    }
                    result = rx.recv() => {
                         let (msg,other_addr) = result.unwrap();
                         if addr != other_addr{
                            writer.write_all(msg.as_bytes()).await.unwrap();
                         }  
                    }
                }

            }
        });
    }
    
}
