use std::io;

fn main() {
	let mut n = String::new();
	io::stdin().read_line(&mut n).unwrap();
	let n:i32 = n.trim().parse().unwrap();
	let mut f = 1;
	let mut f1 = 1;
	let mut f2 = 0;
	for _ in 1..n {
		f = f1 + f2;
		f2 = f1;
		f1 = f;
	}
	println!("{}", f);
}
