fn add(a: u64, b: u64) -> u64 {
    a + b
}

fn bugged_add(a: u64, b: u64) -> u64 {
    a * b
}

fn multiply(&a: &u64, &b: &u64) -> u64 {
    a * b
}

fn spacing() {
    println!("\n\n");
}

fn main() {
    spacing();

    println!("Summing {} + {} = {}", 2, 3, add(2, 3));
    println!("Summing Wrong {} + {} = {}", 2, 3, bugged_add(2, 3));

    let a = 3;
    let b = 4;
    println!("Multipling {} * {} = {}", a, b, multiply(&a, &b)); // No need to borrow to println! macro?

    spacing();

    let base = 8;
    for n in 1..=10 {
        println!("Multipling {} * {} = {}", n, base, base * n);
    }

    spacing();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(1, 2), 3)
    }

    #[test]
    fn test_bugged_add() {
        assert_eq!(bugged_add(1, 2), 3) // This test fails
    }

    #[test]
    fn test_add_again() {
        assert_eq!(add(1, 2), 3)
    }

    #[test]
    fn mult_by_zero() {
        assert_eq!(multiply(&0, &30), 0)
    }

    #[test]
    fn mult_by_ref() {
        assert_eq!(multiply(&10, &10), 100)
    }
}
