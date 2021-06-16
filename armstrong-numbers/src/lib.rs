pub fn is_armstrong_number(n: u32) -> bool {
    let digits: Vec<u32> = n.to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect();

    let mut sum = 0;

    for i in digits {
        sum = sum + i.pow(digits.len() as u32);
    }

    if sum == n {
        true
    } else {
        false
    }
}

// 153 = 1^3 + 5^3 + 3^3 = 1 + 125 + 27 = 153