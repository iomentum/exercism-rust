pub fn square(s: u32) -> u64 {
    if s == 0 || s > 64 {
        panic!("Square must be between 1 and 64 : received {}", s);
    }

    (2 as u64).pow(s - 1).into()
}

pub fn total() -> u64 {
    let mut sum :u64 = 0;

    for i in 1..=64 {
        sum = sum + square(i);
    }

    sum
}
