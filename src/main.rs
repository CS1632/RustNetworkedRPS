#![allow(unused)]
use std::io::prelude::*;
use std::io::{self, Read, Write, Error};
use std::str::FromStr;
use std::thread;
use std::net::{TcpListener, TcpStream};
use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

#[derive(PartialEq, Debug)]
pub enum Weapon{
	Rock,
	Paper,
	Scissors
}

fn main() {//-> std::io::Result<()> {

	//start_tcp_listener();
	//test_battle_logic();
	loop {
		battle_robot();
	}

}

/**
* Allows user to battle computer
*/
fn battle_robot(){
	battle(grab_input(), rand::random());
}


fn grab_input() -> Weapon{
	println!("Rock, Paper, or Scissors?:");

	let mut input = String::new();
	match io::stdin().read_line(&mut input){
		Ok(_) => {
			let choice = Weapon::from_str(&input.to_uppercase()).unwrap();
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
        match input {
            "ROCK" => Ok(Weapon::Rock),
            "PAPER"  => Ok(Weapon::Paper),
            "SCISSORS"  => Ok(Weapon::Scissors),
            _  => Err(()),
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
fn start_tcp_listener() {
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
	println!("Incoming connection from: {}", stream.peer_addr()?);
	let mut buf = [0;512];
	loop {
		let bytes_read = stream.read(&mut buf)?;
		if bytes_read == 0 {return Ok(())}
		stream.write(&buf[..bytes_read])?;
	}
}