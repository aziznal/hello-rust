fn main() {
    println!("Hello, world!");
    println!("I'm a Rustacean!");
}

fn sum(a: u32, b: u32) -> u32 {
    return a + b;
}

fn sum_float(a: f32, b: f32) -> f32 {
    return a + b;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() {
        assert_eq!((), main());
    }

    #[test]
    fn test_sum() {
        assert_eq!(sum(1, 2), 3);
        assert_ne!(sum(1, 2), 4);
    }

    #[test]
    fn test_sum_float() {
        assert_eq!(sum_float(0.1, 0.1), 0.2);
        assert_eq!(sum_float(0.1, 0.2), 0.3);
    }
}
