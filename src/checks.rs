/// <https://en.wikipedia.org/wiki/The_Hitchhiker%27s_Guide_to_the_Galaxy>
pub fn answer(n: u32) -> bool {
    return n == 42;
}

/// <https://en.wikipedia.org/wiki/Composite_number>
pub fn composite(n: u32) -> bool {
    if n <= 3 {
        return false;
    }

    if n % 2 == 0 || n % 3 == 0 {
        return true;
    }

    let mut i: u32 = 5;
    while i * i <= n {
        if n % i == 0 || n % (i + 2) == 0 {
            return true;
        };
        i += 6
    }

    return false;
}

/// <https://en.wikipedia.org/wiki/Cube_(algebra)>
pub fn cube(n: u32) -> bool {
    let cube_root: u32 = f64::from(n).cbrt().floor() as u32;
    let cubed_cube_root: u32 = cube_root * cube_root * cube_root;
    return cubed_cube_root == n;
}

/// <https://en.wikipedia.org/wiki/Parity_(mathematics)>
pub fn even(n: u32) -> bool {
    return n % 2 == 0;
}

/// <https://en.wikipedia.org/wiki/Parity_(mathematics)>
pub fn odd(n: u32) -> bool {
    return n % 2 != 0;
}

/// <https://en.wikipedia.org/wiki/Factorial>
pub fn factorial(n: u32) -> bool {
    if n < 1 {
        return false;
    }

    if n == 1 {
        return true;
    }

    let mut i: u32 = 1;
    let mut m: u32 = n;
    loop {
        if m % i == 0 {
            m = (f64::from(m) / f64::from(i)).floor() as u32;
            i += 1;
        } else {
            return m == 1;
        }
    }
}

/// <https://en.wikipedia.org/wiki/Fibonacci_number>
pub fn fibonacci(n: u32) -> bool {
    return square(5 * n * n + 4) || square(5 * n * n - 4);
}

/// <https://en.wikipedia.org/wiki/Prime_number>
pub fn prime(n: u32) -> bool {
    if n == 2 || n == 3 {
        return true;
    }

    if n <= 1 || n % 2 == 0 || n % 3 == 0 {
        return false;
    }

    let mut i: u32 = 5;
    while i * i <= n {
        if n % i == 0 || n % (i + 2) == 0 {
            return false;
        }
        i += 6
    }
    return true;
}

/// <https://en.wikipedia.org/wiki/Square_number>
pub fn square(n: u32) -> bool {
    let square_root: u32 = f64::from(n).sqrt().floor() as u32;
    return square_root * square_root == n;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_answer() {
        assert_eq!(answer(42), true);
        assert_eq!(answer(6 * 9), false);
    }

    #[test]
    fn test_composite() {
        assert_eq!(composite(0), false);
        assert_eq!(composite(1), false);
        assert_eq!(composite(2), false);
        assert_eq!(composite(3), false);
        assert_eq!(composite(4), true);
        assert_eq!(composite(u32::MAX), true);
    }

    #[test]
    fn test_cube() {
        assert_eq!(cube(0), true);
        assert_eq!(cube(1), true);
        assert_eq!(cube(2), false);
        assert_eq!(cube(8), true);
    }

    #[test]
    fn test_even() {
        assert_eq!(even(0), true);
        assert_eq!(even(1), false);
        assert_eq!(even(2), true);
        assert_eq!(even(100), true);
    }

    #[test]
    fn test_odd() {
        assert_eq!(odd(0), false);
        assert_eq!(odd(1), true);
        assert_eq!(odd(2), false);
        assert_eq!(odd(101), true);
    }

    #[test]
    fn test_factorial() {
        assert_eq!(factorial(0), false);
        assert_eq!(factorial(1), true);
        assert_eq!(factorial(2), true);
        assert_eq!(factorial(3), false);
        assert_eq!(factorial(5), false);
        assert_eq!(factorial(6), true);
        assert_eq!(factorial(100), false);
        assert_eq!(factorial(120), true);
    }

    #[test]
    fn test_fibonacci() {
        assert_eq!(fibonacci(0), true);
        assert_eq!(fibonacci(1), true);
        assert_eq!(fibonacci(2), true);
        assert_eq!(fibonacci(3), true);
        assert_eq!(fibonacci(4), false);
        assert_eq!(fibonacci(5), true);
        assert_eq!(fibonacci(100), false);
        assert_eq!(fibonacci(144), true);
    }

    #[test]
    fn test_prime() {
        assert_eq!(prime(0), false);
        assert_eq!(prime(1), false);
        assert_eq!(prime(2), true);
        assert_eq!(prime(3), true);
        assert_eq!(prime(4), false);
        assert_eq!(prime(5), true);
        assert_eq!(prime(u32::MAX), false);
    }

    #[test]
    fn test_square() {
        assert_eq!(square(0), true);
        assert_eq!(square(1), true);
        assert_eq!(square(2), false);
        assert_eq!(square(3), false);
        assert_eq!(square(4), true);
    }
}
