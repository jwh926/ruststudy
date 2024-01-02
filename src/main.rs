use std::collections::HashMap;

fn main() {
    let mut v = vec![13, 21, 45, 9, 456, 13];

    let avg: i32 = v.iter().sum::<i32>() / v.len() as i32;
    println!("{}", avg);

    v.sort();
    println!("{}", v[v.len() / 2]);

    let mut h: HashMap<_, _> = HashMap::new();
    for i in v {
        let count = h.entry(i).or_insert(0);
        *count += 1;
    }
    println!("{:?}", h);
}
