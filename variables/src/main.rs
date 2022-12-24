use std::io;

fn main() {
	let mut temp = String::new();
	io::stdin().read_line(&mut temp).unwrap();
	let mut temp = temp.trim().split_whitespace();
	let temperature:f64 = temp.next().unwrap().parse().unwrap();
	let method = temp.next().unwrap();
	if method == "F" {
		println!("{} C", (temperature-32.0)/1.8);
	}
	else if method == "C" {
		println!("{} F", (temperature*1.8)+32.0);
	}
	else {
		println!("method is wrong: F or C");
	}
}
