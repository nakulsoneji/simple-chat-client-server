use std::net::{TcpStream, TcpListener};
use std::io::prelude::*;

fn username(stream: &TcpStream) -> String {
    return format!("user-{}", stream.peer_addr().unwrap().port().to_string());
}
fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8000")?;
    let _ = listener.set_nonblocking(true)?;

    let mut streams: Vec<TcpStream> = vec![];

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                stream.set_nonblocking(true)?;
                streams.push(stream);
            }
            Err(_) => {}
        }
        for mut s in &streams {
            let buf: &mut [u8; 127] = &mut [0;127];
            match s.read(buf) {
                Ok(m) => {
                    println!("{}: {}", username(s), std::str::from_utf8(&buf[0..m]).unwrap().trim());
                }
                Err(_) => {}
            }
        }
    }

    Ok(())
}
