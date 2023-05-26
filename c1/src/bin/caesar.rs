fn caesar_cipher(text: &str, shift: i16) -> String {
    let code_a = 'a' as i16;
    let code_z = 'z' as i16;
    let mut result = String::new();

    for ch in text.chars() {
        let mut code = ch as i16;
        if code_a <= code && code <= code_z {
            code = (code - code_a+shift+26) % 26 + code_a;
        }

        result.push((code as u8) as char)
    }
    result
}

fn main() {
    let enc = caesar_cipher("rust is a programming language", 3);
    let dec = caesar_cipher(&enc, -3);
    println!("enc: {}\ndec: {}", enc, dec);
}