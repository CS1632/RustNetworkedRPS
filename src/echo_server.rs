use std::io::{Read, Write, Error};
use std::thread;
use std::net::{TcpListener, TcpStream};


fn main() {
	let listener = TcpListener::bind("0.0.0.0:8888")
		.expect("Could not bind");
		for stream in listener.incoming() {
			println!("stream in listener.incoming()");
			match stream {
				Err(e) => { eprintln!("failed: {}", e)}
				Ok(stream) => {
					println!("Running Ok(stream)");
					thread::spawn(move || {
						handle_client(stream).unwrap_or_else(|error| println!("{:?}", error));
					});
				}
			}
		}
}

fn handle_client(mut stream: TcpStream) -> Result<(), Error>{
	println!("Incoming connection from: {}", stream.peer_addr()?);
	println!("Running handle_client()");
	let mut buf = [0;512];
	let my_string: &str = "some string";
	let my_bytes: &[u8] = my_string.as_bytes();
	loop {
		let bytes_read = stream.read(&mut buf)?;
		if bytes_read == 0 {return Ok(())}
		stream.write(&buf[..bytes_read])?;
	}
}