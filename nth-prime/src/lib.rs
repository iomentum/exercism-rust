use std::convert::TryInto;

pub fn is_prime(tested_int: u32) -> bool {
    for i in 2..=(tested_int as f64).sqrt() as u32 {
        dbg!(i);
        if tested_int % i == 0 {
            return false;
        }
    }
    return true;
}

pub fn nth(n: u32) -> u32 {
    let mut prime_list: Vec<u32> = vec![];

    let mut i = 2;

    loop {
        if is_prime(i) {
            prime_list.push(i);
        }

        if prime_list.len() > n.try_into().unwrap() {
            break;
        }

        i = i + 1;
    }

    prime_list.pop().unwrap()
}
