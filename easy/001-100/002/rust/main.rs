fn main() {
    println!("Hello, world!");
}

fn tip_percentage(bill: f64, tip_percent: f64) -> f64 {
    bill * tip_percent
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tip_percentage() {
        assert_eq!(tip_percentage(10.00, 0.15), 1.50);
        assert_eq!(tip_percentage(10.00, 0.20), 2.00);
        assert_eq!(tip_percentage(10.00, 0.25), 2.50);
    }
}
