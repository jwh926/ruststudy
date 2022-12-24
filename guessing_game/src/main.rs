use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
	println!("Welcome to Guessing game");

	let secret = rand::thread_rng().gen_range(1..101);
	
	println!("guess the number");
	loop {
		let mut guess = String::new();
		io::stdin().read_line(&mut guess).expect("error");
		let guess: i32 = match guess.trim().parse() {
			Ok(num) => num,
			Err(_) => {
				println!("fuck u");
				continue;
			},
		};

		match guess.cmp(&secret) {
			Ordering::Less => println!("up"),
			Ordering::Greater => println!("down"),
			Ordering::Equal => {
				println!("correct!");
				break;
			},
		}
	}
}
