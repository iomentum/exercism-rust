use std::convert::TryInto;

fn is_prime(nb: u32) -> bool {
    if nb == 2 {
        return true;
    }

    for j in 2..nb {
        if nb % j == 0 {
            return false;
        }
    }
    return true;
}

pub fn nth(n: u32) -> u32 {
    let mut primes = Vec::new();

    let mut i: u32 = 2;
    while primes.len() <= n.try_into().unwrap() {
        if is_prime(i) {
            primes.push(i);
        }
        i = i + 1;
    }

    primes.pop().unwrap()
}
