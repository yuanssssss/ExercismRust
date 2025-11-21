/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    // todo!("Is the Luhn checksum for {code} valid?");

    let code_str = String::from(code);
    let code_str = code_str.chars().filter(|ch| !ch.is_whitespace()).collect::<String>();
    // println!("{}",code_str);
    
    if code_str.len() <=1 {
        return false
    }
    let res = code_str.chars().map(|ch| ch.is_numeric()).all(|r| r);
    if res != true {
        return false;
    }
    
    let mut res = 0;
    for (i, ch) in code_str.chars().rev().enumerate() {
        let mut num = ch.to_digit(10).unwrap();
        if i%2 ==0 {
            res +=num;
            continue;
        }
        num *= 2;
        if num >= 10 {
            num -=9;
        }
        res += num;
    }
    // println!("checksum is {}", res);
    res %10 ==0
}
