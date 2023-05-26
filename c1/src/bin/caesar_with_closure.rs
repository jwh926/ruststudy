fn caesar_cipher(text: &str, shift: i16) -> String {
    let a = 'a' as i16;
    let is_az = |c| 'a' <= c && c <= 'z';
    let conv = |c| (((c - a + shift + 26) % 26 + a) as u8) as char;
    let enc1 = |c| if is_az(c) { conv(c as i16) } else { c };
    text.chars().map(|c| enc1(c)).collect()
}

fn main() {
    let enc = caesar_cipher("rust is a programming language", 3);
    let dec = caesar_cipher(&enc, -3);
    println!("enc: {}\ndec: {}", enc, dec);
}
