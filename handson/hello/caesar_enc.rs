fn encrypt(text: &str, shift: i16) -> String {
    // 'A' の文字コードを i16 で得る！（これでいけるのか。）
    let code_a = 'A' as i16;
    let code_z = 'Z' as i16;

    let mut result = String::new();
    for ch in text.chars() {
        let mut c = ch as i16;
        if code_a <= c && c <= code_z {
            // -25 % 26 = -25 となるため、26 を足して正の値にしてから mod 26 する。
            // python では -25 % 26 = 1 となる！
            c = (c - code_a + shift + 26) % 26 + code_a;
        }

        result.push((c as u8) as char);
    }

    return result;
}

fn main() {
    let a = -25 % 26;
    println!("-25 % 26 = {}", a);

    let enc = encrypt("I LOVE YOU", 3);
    let dec = encrypt(&enc, -3);
    println!("{} => {}", enc, dec);
}
