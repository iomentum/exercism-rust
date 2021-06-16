fn is_multiple(nb: u32, factors: &[u32]) -> bool {
    for factor in factors {
        if *factor != 0 as u32 && nb % factor == 0 {
            return true;
        }
    }
    
    false
}

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut multiple_sum = 0;
    
    for i in 1..limit {
        if is_multiple(i, factors) {
            multiple_sum = multiple_sum + i
        }
    }

    multiple_sum
}
