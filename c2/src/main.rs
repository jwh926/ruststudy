fn main() {
	let s = "3.141592a";
	let num = s.parse::<f64>().expect("변환 실패");
	println!("{:.2}", num);
}