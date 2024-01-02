fn main() {
    let mut v = 10;
    set_value(&mut v);
    println!("{}", v);
}

fn set_value(arg: &mut i32) {
    *arg = 123;
}
