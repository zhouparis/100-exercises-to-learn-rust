// Given a number `n`, return the `n+1`th number in the Fibonacci sequence.
//
// The Fibonacci sequence is defined as follows:
//
// - The first number of the sequence is 0.
// - The second number of the sequence is 1.
// - Every subsequent number is the sum of the two preceding numbers.
//
// So the sequence goes: 0, 1, 1, 2, 3, 5, 8, 13, 21, and so on.
//
// We expect `fibonacci(0)` to return `0`, `fibonacci(1)` to return `1`,
// `fibonacci(2)` to return `1`, and so on.
pub fn fibonacci(n:usize) -> usize {
    let mut memo: Vec<Option<usize>> = vec![None; n + 1];

    if n >= 0 { memo[0] = Some(0)};
    if n >= 1 { memo[1] = Some(1)};

    fn fib_helper(n: usize, memo: &mut Vec<Option<usize>>) -> usize {
        if let Some(val) = memo[n] {
            return val
        } else {
            let value = fib_helper(n-1, memo) + fib_helper(n-2, memo);
            memo[n] = Some(value);
            return value
        }
    } 
    return fib_helper(n, &mut memo)
}
#[cfg(test)]
mod tests {
    use crate::fibonacci;

    #[test]
    fn first() {
        assert_eq!(fibonacci(0), 0);
    }

    #[test]
    fn second() {
        assert_eq!(fibonacci(1), 1);
    }

    #[test]
    fn third() {
        assert_eq!(fibonacci(2), 1);
    }

    #[test]
    fn tenth() {
        assert_eq!(fibonacci(10), 55);
    }

    #[test]
    fn thirtieth() {
        assert_eq!(fibonacci(30), 832040);
    }
}
