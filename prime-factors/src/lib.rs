

pub fn factors(n: u64) -> Vec<u64> {
    let mut factors = Vec::new();
    let mut acc = n as f64;

    for i in 2..=n {
        let f = i as f64;

        if acc / f == 1.0 { 
            factors.push(i);
            break;
        }
        
        while (acc / f).fract() == 0.0 {
            factors.push(i);
            acc = acc / f;
        }
    }

    factors
}
