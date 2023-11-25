use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

const PORT: usize = 8000;

fn username(stream: &TcpStream) -> String {
    return format!("{}", stream.peer_addr().unwrap().to_string());
}

fn send_to_streams(streams: &Vec<TcpStream>, data: &[u8]) {
    for mut sw in streams.iter() {
        match sw.write(data) {
            Ok(_) => {} 
            Err(_) => {
                panic!("error!");
            }
        }
    }
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind(format!("127.0.0.1:{PORT}"))?;
    println!("listening on {}", listener.local_addr().unwrap().to_string());
    let _ = listener.set_nonblocking(true)?;
    let mut streams: Vec<TcpStream> = vec![];

    for connection in listener.incoming() {
        match connection {
            Ok(stream) => {
                stream.set_nonblocking(true)?;
                println!("client connected: {}", stream.peer_addr()?.to_string());
                streams.push(stream);
            }
            Err(_) => {}
        }
        
        let mut i = 0;
        while i < streams.len() {      
            let buf: &mut [u8; 127] = &mut [0; 127];
            let s = &mut streams[i];

            match s.read(buf) {
                Ok(0) => {
                    println!("client disconnected: {}", s.peer_addr()?.to_string());
                    streams.remove(i);
                }
                Ok(m) => {
                    println!(
                        "{}: {:?} ({})",
                        username(s),
                        &buf[0..m],
                        std::str::from_utf8(&buf[0..m]).unwrap().trim()
                    );
                    send_to_streams(&mut streams, buf);
                }
                Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {}
                Err(_) => {
                    println!("error");
                }
            }
            i += 1;
        }
    }
    
    Ok(())
}