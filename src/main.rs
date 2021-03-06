use std::fs::File;
use std::io::prelude::*;
use std::io::{self, Read};
use std::net::{Shutdown, TcpListener, TcpStream};

fn handle_client(mut stream: TcpStream) {
    println!("Got connection");

    let mut file = File::options()
        .write(true)
        .read(true)
        .open("/sys/kernel/debug/sof/fw_gdb")
        .expect("opening fw_gdb file failed");

    stream
        .set_nonblocking(true)
        .expect("set_nonblocking call failed");

    let mut gdb_buf = [0 as u8; 128];
    let mut tcp_buf = [0 as u8; 128];
    loop {
        match stream.read(&mut tcp_buf) {
            Ok(size) => {
                if size == 0 {
                    // Client disconnected
                    break;
                }
                // Write data to fw gdb file
                // write_all doesn't work here because it fails on write_zero, which can happen when GDB ring buffer is full
                let mut buf = &tcp_buf[..size];
                while !buf.is_empty() {
                    let count = match file.write(buf) {
                        Ok(c) => c,
                        Err(e) if e.kind() == io::ErrorKind::WriteZero => 0,
                        Err(e) => panic!("Writing to fw_gdb file failed: {}", e.to_string()),
                    };
                    buf = &buf[count..]
                }
            }
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                // If there's no data, read from file and send any data back over TCP
                let count = file.read(&mut gdb_buf).unwrap();
                if count > 0 && gdb_buf[0] != b'\0' {
                    stream.write_all(&gdb_buf[0..count]).unwrap();
                }
            }
            Err(e) => {
                println!("{:?}", e);
                println!(
                    "An error occurred, terminating connection with {}",
                    stream.peer_addr().unwrap()
                );
                stream.shutdown(Shutdown::Both).unwrap();
                break;
            }
        }
    }
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:4000")?;

    println!("Ready on port 4000");

    for stream in listener.incoming() {
        handle_client(stream?);
    }
    Ok(())
}
