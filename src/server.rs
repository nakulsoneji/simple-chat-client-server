use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

fn username(stream: &TcpStream) -> String {
    return format!("{}", stream.peer_addr().unwrap().to_string());
}

fn send_to_streams(streams: &mut Vec<TcpStream>, data: &[u8]) {
    for sw in streams.iter_mut() {
        match sw.write(data) {
            Ok(_) => {} 
            Err(_) => {
                panic!("error!");
            }
        }
    }
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8000")?;
    let _ = listener.set_nonblocking(true)?;

    let mut streams: Vec<TcpStream> = vec![];

    loop {
        let connection = listener.accept();
        match connection {
            Ok((stream, _socket)) => {
                stream.set_nonblocking(true)?;
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
                    println!("client disconnected: {}", s.peer_addr().unwrap().to_string());
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
                    println!("here")
                    
                }
                Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {}
                Err(_) => {
                    println!("error");
                }
            }
            i += 1;
        }
    }
}
