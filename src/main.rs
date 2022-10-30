#![allow(unused)]
use std::io::{self, prelude::*, BufReader, Read, Write, Error};
use std::str::{self, from_utf8, FromStr};
use std::net::{TcpListener, TcpStream, Shutdown};
use rand::{distributions::{Distribution, Standard}, Rng};
use std::sync::mpsc::*;
use std::thread;



#[derive(PartialEq, Debug)]
pub enum Weapon{
	Rock,
	Paper,
	Scissors
}

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum Role{
	Host,
	Client,
}



fn main() {
	let role = get_role();
	let (mut tx, mut rx): (Sender<Weapon>, Receiver<Weapon>) = channel();
	match role {
		Role::Host => {
			thread::spawn(|| {
				host();
			});
		}
		Role::Client => {
		//client_available = true;
			thread::spawn(|| {
				client();
			});
		}
	}
	loop {
		//battle_robot();
		battle_human(role);
	}
}

/**
 * Asks user their server role
 */
fn get_role() -> Role{
	println!("Do you wanna be the host or a client?");
	let mut input = String::new();
	match io::stdin().read_line(&mut input){
		Ok(_) => {
			let choice = Role::from_str(&input).unwrap();
			return choice;
		}
		Err(e) => {
			println!("Try again");
			return get_role();
		}
	}
}

/**
* Allows user to battle computer
*/
fn battle_robot(){
	println!("Rock, paper, or scissors?");
	battle(Weapon::from_str(&get_input()).unwrap(), rand::random());
}


/**
 * Host and client can battle
 */
fn battle_human(role: Role){
	//enter your weapon type
	get_input();

	//host goes first...
	//then client goes...
	//and then battle


	
}




/**
 *  Randomly generates a weapon. Each outcome has an equal opportunity to happen.
 */
impl Distribution<Weapon> for Standard {
	
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Weapon {
    	match rng.gen_range(0..=2) {
            0 => Weapon::Rock,
            1 => Weapon::Paper,
            _ => Weapon::Scissors,
        }
    }
}

/**
* Turns str into Weapon 
*/
impl FromStr for Weapon {

    type Err = ();

    fn from_str(input: &str) -> Result<Weapon, Self::Err> {
		let tu = input.to_uppercase();
		let trim = tu.trim();
        match trim {
            "ROCK" => Ok(Weapon::Rock),
            "PAPER"  => Ok(Weapon::Paper),
            "SCISSORS"  => Ok(Weapon::Scissors),
            _  => Err(()),
        }
    }
}

/**
* Turns str into Role
*/
impl FromStr for Role {
    type Err = ();
    fn from_str(input: &str) -> Result<Role, Self::Err> {
		let tu = input.to_uppercase();
		let trim = tu.trim();
        match trim {
            "HOST"  => Ok(Role::Host),
			//by default, you will be a client
            _  => Ok(Role::Client),
        }
    }
}

/**
 * Runs 100 randomly generated battles to test the battle logic
 */
fn test_battle_logic(){
	for _i in 1..100 {
		battle(rand::random(), rand::random());
	};
}


/* Battles rps */
fn battle(my_weapon: Weapon, opp_weapon: Weapon){
	if my_weapon == Weapon::Rock {
		if opp_weapon == Weapon::Rock {
			println!("{:?} ties {:?}", my_weapon, opp_weapon);
		}
		if opp_weapon == Weapon::Paper {
			println!("{:?} is covered by {:?}", my_weapon, opp_weapon);
		}
		if opp_weapon == Weapon::Scissors {
			println!("{:?} breaks {:?}", my_weapon, opp_weapon);
		}
	}
	if my_weapon == Weapon::Paper {
		if opp_weapon == Weapon::Rock {
			println!("{:?} covers {:?}", my_weapon, opp_weapon);
		}
		if opp_weapon == Weapon::Paper {
			println!("{:?} ties {:?}", my_weapon, opp_weapon);
		}
		if opp_weapon == Weapon::Scissors {
			println!("{:?} is cut by {:?}", my_weapon, opp_weapon);
		}
	}
	if my_weapon == Weapon::Scissors {
		if opp_weapon == Weapon::Rock {
			println!("{:?} is crushed by {:?}", my_weapon, opp_weapon);
		}
		if opp_weapon == Weapon::Paper {
			println!("{:?} cuts {:?}", my_weapon, opp_weapon);
		}
		if opp_weapon == Weapon::Scissors {
			println!("{:?} ties {:?}", my_weapon, opp_weapon);
		}
	}
}



/**
 * Sets up TCP listener
 */
fn host() {
	//I chose 9800 because that's my birthday, other than that, the specific port number isn't import.
    let listener = TcpListener::bind("0.0.0.0:9800").unwrap();
    // accept connections and process them, spawning a new thread for each one
    //println!("Server listening on port 9800");
	println!("Waiting for client to join...");
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
				//Connection succeeded
                println!("Player 2 connected from: {}", stream.peer_addr().unwrap());
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


/**
 * Called from host() to handle data recieved from TCP Stream
 */
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


fn client() {
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
					Err(_e) => {
						println!("Heard no response from server");
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