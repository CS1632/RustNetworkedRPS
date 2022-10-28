use std::net::{TcpStream};
use std::io;
use std::io::{Read, Write};
use std::str::from_utf8;

fn main() {
	let mut host = TcpStream::connect("127.0.0.1:9800");
	loop {
		match host {
			Ok(ref mut stream) => {
				println!("Successfully connected to server in port 9800");
				let msg: String = get_input();
				stream.write(msg.as_bytes()).unwrap();

				let mut data = vec![0 as u8; msg.len()]; //sets buffer to be the length of the message
				match stream.read_exact(&mut data) {
					Ok(_) => {
						if !data.eq(msg.as_bytes()) {
							println!("Unexpected reply: {}", from_utf8(&data).unwrap());
						}
					},
					Err(e) => {
						println!("Heard no response from server", e);
					}
				}
			},
			Err(ref e) => {
				println!("Failed to connect: {}", e);
				break;
			}
		}
	}	
	println!("Connection terminated");
}


fn get_input() -> String {
	println!("Rock, Paper, or Scissors?:");

	let mut input: String = String::new();
	match io::stdin().read_line(&mut input){
		Ok(_) => {
			return input.trim().to_string();
		}
		Err(_e) => {
			return String::from("Error");
		}
	}
}