type SequenceMember = fn(u32) -> u32;

type SequenceNext = fn(Vec<u32>, u32) -> u32;

fn iterate_member(fn_member: SequenceMember, start: u32, length: u32) -> Vec<u32> {
    let end: u32 = start + length;
    let mut items: Vec<u32> = Vec::new();
    (start..end).for_each(|x: u32| {
        let member: u32 = fn_member(x);
        items.push(member);
    });
    return items;
}

fn iterate_next(initial: Vec<u32>, fn_next: SequenceNext, start: u32, length: u32) -> Vec<u32> {
    let end: u32 = start + length;
    let mut items: Vec<u32> = Vec::from(initial);
    (start..end).for_each(|x: u32| {
        let next: u32 = fn_next(items.clone(), x);
        items.push(next);
    });
    return items;
}

/// http://oeis.org/A000578
pub fn cube_member(nth: u32) -> u32 {
    return nth * nth * nth;
}

/// http://oeis.org/A000578
pub fn cube(nth: u32, length: u32) -> Vec<u32> {
    return iterate_member(cube_member, nth, length);
}

/// http://oeis.org/A005843
pub fn even_member(nth: u32) -> u32 {
    return 2 * nth;
}

/// http://oeis.org/A005843
pub fn even(nth: u32, length: u32) -> Vec<u32> {
    return iterate_member(even_member, nth, length);
}

/// http://oeis.org/A000142
pub fn factorial_member(nth: u32) -> u32 {
    if nth == 0 {
        return 1;
    }
    return nth * factorial_member(nth - 1);
}

/// http://oeis.org/A000142
pub fn factorial_next(members: Vec<u32>, nth: u32) -> u32 {
    return members[members.len() - 1] * nth;
}

/// http://oeis.org/A000142
pub fn factorial(nth: u32, length: u32) -> Vec<u32> {
    let initial: Vec<u32> = Vec::from([factorial_member(nth)]);
    return iterate_next(initial, factorial_next, nth + 1, length - 1);
}

/// http://oeis.org/A000045
pub fn fibonacci_member(nth: u32) -> u32 {
    if nth == 0 {
        return 0;
    }
    if nth == 1 {
        return 1;
    }
    return fibonacci_member(nth - 1) + fibonacci_member(nth - 2);
}

/// http://oeis.org/A000045
pub fn fibonacci_next(members: Vec<u32>, _: u32) -> u32 {
    return members[members.len() - 2] + members[members.len() - 1];
}

/// http://oeis.org/A000045
pub fn fibonacci(nth: u32, length: u32) -> Vec<u32> {
    let initial: Vec<u32> = Vec::from([fibonacci_member(nth), fibonacci_member(nth + 1)]);
    return iterate_next(initial, fibonacci_next, nth + 2, length - 2);
}

/// http://oeis.org/A000027
pub fn natural_member(nth: u32) -> u32 {
    return nth;
}

/// http://oeis.org/A000027
pub fn natural(nth: u32, length: u32) -> Vec<u32> {
    return iterate_member(natural_member, nth, length);
}

/// http://oeis.org/A005408
pub fn odd_member(nth: u32) -> u32 {
    return 2 * nth + 1;
}

/// http://oeis.org/A005408
pub fn odd(nth: u32, length: u32) -> Vec<u32> {
    return iterate_member(odd_member, nth, length);
}

/// https://oeis.org/A000079
pub fn powers2_member(nth: u32) -> u32 {
    return 2u32.pow(nth);
}

/// https://oeis.org/A000079
pub fn powers2_next(members: Vec<u32>, _: u32) -> u32 {
    return members[members.len() - 1] * 2;
}

/// https://oeis.org/A000079
pub fn powers2(nth: u32, length: u32) -> Vec<u32> {
    let initial: Vec<u32> = Vec::from([powers2_member(nth)]);
    return iterate_next(initial, powers2_next, nth + 1, length - 1);
}

/// http://oeis.org/A000290
pub fn square_member(nth: u32) -> u32 {
    return nth * nth;
}

/// http://oeis.org/A000290
pub fn square(nth: u32, length: u32) -> Vec<u32> {
    return iterate_member(square_member, nth, length);
}

/// http://oeis.org/A003154
pub fn star_member(nth: u32) -> u32 {
    if nth == 0 {
        return 0;
    }
    return 6 * nth * (nth - 1) + 1;
}

/// http://oeis.org/A003154
pub fn star(nth: u32, length: u32) -> Vec<u32> {
    return iterate_member(star_member, nth, length);
}

/// https://oeis.org/A000217
pub fn triangular_member(nth: u32) -> u32 {
    return (nth * (nth + 1)) / 2;
}

/// https://oeis.org/A000217
pub fn triangular_next(members: Vec<u32>, nth: u32) -> u32 {
    return members[members.len() - 1] + nth;
}

/// https://oeis.org/A000217
pub fn triangular(nth: u32, length: u32) -> Vec<u32> {
    let initial: Vec<u32> = Vec::from([triangular_member(nth)]);
    return iterate_next(initial, triangular_next, nth + 1, length - 1);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cube() {
        assert_eq!(cube(1, 10), [1, 8, 27, 64, 125, 216, 343, 512, 729, 1000]);
    }

    #[test]
    fn test_even() {
        assert_eq!(even(0, 10), [0, 2, 4, 6, 8, 10, 12, 14, 16, 18]);
    }

    #[test]
    fn test_factorial() {
        assert_eq!(
            factorial(0, 10),
            [1, 1, 2, 6, 24, 120, 720, 5040, 40320, 362880]
        );
    }

    #[test]
    fn test_fibonacci() {
        assert_eq!(fibonacci(0, 10), [0, 1, 1, 2, 3, 5, 8, 13, 21, 34]);
    }

    #[test]
    fn test_natural() {
        assert_eq!(natural(0, 10), [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }

    #[test]
    fn test_odd() {
        assert_eq!(odd(0, 10), [1, 3, 5, 7, 9, 11, 13, 15, 17, 19]);
    }

    #[test]
    fn test_powers2() {
        assert_eq!(powers2(0, 10), [1, 2, 4, 8, 16, 32, 64, 128, 256, 512]);
    }

    #[test]
    fn test_square() {
        assert_eq!(square(0, 10), [0, 1, 4, 9, 16, 25, 36, 49, 64, 81]);
    }

    #[test]
    fn test_star() {
        assert_eq!(star(0, 10), [0, 1, 13, 37, 73, 121, 181, 253, 337, 433]);
    }

    #[test]
    fn test_triangular() {
        assert_eq!(triangular(0, 10), [0, 1, 3, 6, 10, 15, 21, 28, 36, 45]);
    }
}
