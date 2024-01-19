// regchest_client_util.rs
// use: - client util functions for regchest_command
//      - regchest_send: sends command to regchest_server over tcp
// authers: idkky137
//

use std::io::{Read, Write};
use std::net::TcpStream;

pub fn regchest_send(command: &str) {
    if let Ok(mut stream) = TcpStream::connect("127.0.0.1:8080") {
        stream
            .write_all(command.as_bytes())
            .expect("Error: Failed sending command");

        let mut buffer = Vec::new();

        loop {
            let mut chunk = vec![0; 1024];
            match stream.read(&mut chunk) {
                Ok(size) if size > 0 => {
                    buffer.extend_from_slice(&chunk[..size]);
                    let buffer_len = buffer.len();
                    let eor_flag: &[u8] = &buffer[buffer_len - 2..];
                    if eor_flag == [b'\t', b'\t'] {
                        break;
                    }
                }
                Ok(0) | Ok(_) | Err(_) => {
                    println!("Error: Failed to receive response from server");
                    break;
                }
            }
        }

        let response = String::from_utf8_lossy(&buffer);
        println!("{}", response);
    } else {
        eprintln!("Error: failed to connect to server")
    }
}
