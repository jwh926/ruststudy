use num_bigint::BigInt;

fn main() {
	let a = BigInt::from(2);
	println!("{}", a.pow(4000000));
}