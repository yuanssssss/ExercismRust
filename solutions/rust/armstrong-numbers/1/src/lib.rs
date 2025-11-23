

pub fn is_armstrong_number(num: u32) -> bool {
    // todo!("true if {num} is an armstrong number")
    let mut num_copy = num;
    let bits = (num as f64).log10() as usize + 1;
    let mut sum =  0;
    while num_copy != 0 {
        sum += (num_copy %10).pow(bits as u32);
        num_copy /= 10; 
    }
    sum == num
}
