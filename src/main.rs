#![allow(unused)]
use std::io::{prelude::*, BufReader};
use std::io::{self, Read, Write, Error};
use std::str;
use std::str::FromStr;
use std::thread;
use std::net::{TcpListener, TcpStream};
use rand::{
    distributions::{Distribution, Standard},
    Rng,
};
use std::sync::Mutex;

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


static mut MUTEX: Mutex<i32> = Mutex::new(0);
//static mut HOST_W: Weapon;
//static mut CLIENT_W: Weapon;

fn main() {
	let role = get_role();
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
	battle(grab_input(), rand::random());
}


/**
 * Host and client can battle
 */
fn battle_human(role: Role){
	//enter your weapon type
	grab_input();

	//host goes first...
	//then client goes...
	//and then battle


	
}


/**
 * Gets user's weapon
 */
fn grab_input() -> Weapon{
	println!("Rock, Paper, or Scissors?:");

	let mut input = String::new();
	match io::stdin().read_line(&mut input){
		Ok(_) => {
			let choice = Weapon::from_str(&input).unwrap();
			return choice;
		}
		Err(e) => {
			println!("Try again");
			return grab_input();
		}
	}
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
	println!("Setting up server...");
	let listener = TcpListener::bind("0.0.0.0:8888")
		.expect("Could not bind");
		for stream in listener.incoming() {
			match stream {
				Err(e) => { eprintln!("failed: {}", e)}
				Ok(stream) => {
					thread::spawn(move || {
						handle_client(stream).unwrap_or_else(|error| eprintln!("{:?}", error));
					});
				}
			}
		}
}


fn handle_client(mut stream: TcpStream) -> Result<(), Error>{
	println!("Player 2 connecting from: {}", stream.peer_addr()?);
	let mut buf = [0;512];
	loop {
		let bytes_read = stream.read(&mut buf)?;
		if bytes_read == 0 {return Ok(())}
		stream.write(&buf[..bytes_read])?;
	}
}


fn client() {
	let mut stream = TcpStream::connect("127.0.0.1:8888")
	.expect("Could not connect to server");
	loop {
		let mut input = String::new();
		let mut buffer: Vec<u8> = Vec::new();
		io::stdin()
			.read_line(&mut input)
			.expect("Failed to read from stdin");
		//get weapon from client
		let w = Weapon::from_str(&input)
			.expect("Invalid weapon");
		//stream.write(input.as_bytes()).expect("Failed to write to server");

		let mut reader = BufReader::new(&stream);

		reader.read_until(b'\n', &mut buffer).expect("Could not read into buffer");
		print!("{}", str::from_utf8(&buffer).expect("Could not write buffer as string"));
	}
}