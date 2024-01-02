use std::collections::HashMap;

const V_DATA: &str = "a,b,c,a,c,b,c,b,a,c,b,a,a,b,a,b,c,a,b,a,b,c,b,b,c,a,b,b,b,b,b,b";

fn main() {
    let mut c_map = HashMap::new();
    c_map.insert("a", 0);
    c_map.insert("b", 0);
    c_map.insert("c", 0);

    for w in V_DATA.split(',') {
        c_map.insert(w, c_map[w] + 1);
    }

    for k in ["a", "b", "c"] {
        println!("{}: {:>2}", k, c_map[k]);
    }
}
