use std::io::{stdout, Write};

fn main() {
    let height = loop {
        if let Some(h) = input("height: ") {
            break h / 100.0;
        } else {
            println!("error");
            continue;
        }
    };

    let bmi = loop {
        if let Some(w) = input("weight: ") {
            break w / height.powf(2.0);
        } else {
            println!("error");
            continue;
        }
    };

    println!("{}", bmi);
}

fn input(prompt: &str) -> Option<f64> {
    print!("{}", prompt);
    let _ = stdout().flush();
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    s.trim().parse().ok()
}
