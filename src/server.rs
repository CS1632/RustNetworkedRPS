use std::thread;
use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Read, Write};


fn main() {
	host();
}

fn host() {
	//I chose 9800 because that's my birthday, other than that, there's no reason for that.
    let listener = TcpListener::bind("0.0.0.0:9800").unwrap();
    // accept connections and process them, spawning a new thread for each one
    println!("Server listening on port 9800");
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
				 //Connection succeeded
                println!("New connection: {}", stream.peer_addr().unwrap());
                thread::spawn(move|| {
                    handle_client(stream)
                });
            }
            Err(e) => {
				//Connection failed
                println!("Error: {}", e);
            }
        }
    }
    // close the socket server
    drop(listener);
}



fn handle_client(mut stream: TcpStream) {
	//use a 10 char buffer. we don't really need more data for what we're sending.
    let mut data = [0 as u8; 10];
    while match stream.read(&mut data) {
        Ok(size) => {
			if size > 0 {
				//write data to the console
				println!("{}", String::from_utf8((&data[0..size]).to_vec()).unwrap());
				//write data to the stream	
				stream.write(&data[0..size]).unwrap();
			}
			true
		}
        Err(_) => {
            println!("An error occurred, terminating connection with {}", stream.peer_addr().unwrap());
            stream.shutdown(Shutdown::Both).unwrap();
            false
        }
	} {}
}