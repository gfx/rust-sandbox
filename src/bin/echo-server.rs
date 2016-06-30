extern crate threadpool;

use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use threadpool::ThreadPool;

fn main() {
    let n_workers = 8;

    let addr = "127.0.0.1:8124";
    println!("bind: {}", addr);
    let listener = TcpListener::bind(addr).unwrap();

    let executor = ThreadPool::new(n_workers);

    fn handle_client(mut stream: TcpStream) {
        let mut buf = [0; 128];
        loop {
            let len = match stream.read(&mut buf) {
                Ok(0) => break,
                Ok(len) => len,
                Err(ref e) if e.kind() == std::io::ErrorKind::Interrupted => continue,
                Err(e) => {
                    println!("{}", e);
                    break;
                }
            };
            print!("{}", String::from_utf8_lossy(&buf[..len]));
            let _ = std::io::stdout().flush();
            let _ = stream.write_all(&buf[..len]);
        }
    }

    // accept connections and process them, spawning a new thread for each one
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("connection succeeded: {:?}", stream);
                executor.execute(move || {
                    handle_client(stream);
                    println!("connection finished")
                });
            }
            Err(e) => {
                println!("connection failed: {}", e);
            }
        }
    }

    // close the socket server
    drop(listener);
}