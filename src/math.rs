pub fn check_evenness(num: u8) -> bool {
    num % 2 == 0
}

pub fn check_primeness(num: u8) -> bool {
    if num == 1 {
        return false;
    }
    let mut divider = 2;
    let half = num / 2;

    while divider <= half {
        if num % divider == 0 {
            return false;
        }
        divider += 1;
    }

    true
}

pub fn calculator(num1: i16, operator: u8, num2: i16) -> i16 {
    match operator {
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

pub fn find_missing_number_in_progression(progression: Vec<u8>) -> u8 {
    let missing_number_index = progression.iter().position(|&num| num == 0).unwrap();
    let last_index = progression.len() - 1;

    if missing_number_index == 0 || missing_number_index == last_index {
        let second_number = progression[1];
        let third_number = progression[2];
        let step = third_number - second_number;

        if missing_number_index == 0 {
            second_number - step
        } else {
            progression[last_index - 1] + step
        }
    } else {
        let first_number = progression[0];
        let last_number = progression[last_index];
        let quantity = progression.len();
        let step = (last_number - first_number) / (quantity - 1) as u8;

        progression[missing_number_index - 1] + step
    }
}
