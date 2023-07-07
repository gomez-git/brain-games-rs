pub fn check_evenness(num: u8) -> bool {
    num % 2 == 0
}

pub fn calculator(num1: i16, operator: u8, num2: i16) -> i16 {
    match operator as u8 {
        b'+' => num1 + num2,
        b'-' => num1 - num2,
        b'*' => num1 * num2,
        b'%' => num1 % num2,
        _ => 0,
    }
}
