pub fn nth(n: u64) -> u64 {
    let mut a = 0;
    let mut b = 1;

    for _ in 0..n {
        let t = a + b;
        a = b;
        b = t;
    }

    a
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_fibs() {
        assert_eq!(nth(0), 0);
        assert_eq!(nth(1), 1);
        assert_eq!(nth(2), 1);
        assert_eq!(nth(3), 2);
        assert_eq!(nth(4), 3);
        assert_eq!(nth(5), 5);
        assert_eq!(nth(6), 8);
        assert_eq!(nth(7), 13);
        assert_eq!(nth(8), 21);
    }
}
