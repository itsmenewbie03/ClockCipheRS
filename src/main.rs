fn clock_cipher(input: &str) -> String {
    let mut result: Vec<String> = Vec::new();
    for x in input.trim().to_uppercase().chars() {
        let char_code = x as u8;
        let res: String = match char_code {
            00..=31 | 33..=64 | 91.. => "".to_owned(),
            32 => "00".to_owned(),
            65 => "AM".to_owned(),
            66..=89 => {
                let res = char_code - 65;
                res.to_string()
            }
            90 => "PM".to_owned(),
        };
        result.push(res);
    }
    result.join(":")
}

fn main() {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("No Input Provided");
    let result = clock_cipher(&input);
    println!("ClockCipheredText: {result}");
}
