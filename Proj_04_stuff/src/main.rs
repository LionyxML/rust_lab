use std::env;
use std::str::FromStr;

fn great_common_denominator(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        m = m % n;
    }
    n
}

#[test]
fn test_great_common_denominator() {
    assert_eq!(great_common_denominator(14, 15), 1);

    assert_eq!(
        great_common_denominator(2 * 3 * 4 * 5 * 11 * 17, 3 * 7 * 11 * 13 * 19),
        3 * 11
    )
}

fn main() {
    let mut numbers = Vec::new();

    for arg in env::args().skip(1) {
        numbers.push(u64::from_str(&arg).expect("error parsing argument"));
    }

    if numbers.len() == 0 {
        eprintln!("Usage: gcd NUMBER ...");
        std::process::exit(1);
    }

    let mut d = numbers[0];
    for m in &numbers[1..] {
        d = great_common_denominator(d, *m);
    }

    println!("The greatest common divisor of {:?} is {}", numbers, d);
}
