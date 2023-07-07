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

pub fn find_greatest_common_divisor(num1: u8, num2: u8) -> u8 {
    let min = if num1 > num2 { num1 } else { num2 };
    let mut divider = min;

    while divider > 1 {
        if num1 % divider == 0 && num2 % divider == 0 {
            return divider;
        }
        divider -= 1;
    }

    1
}
