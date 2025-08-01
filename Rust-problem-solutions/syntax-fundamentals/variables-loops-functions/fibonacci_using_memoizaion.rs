/*
Why Memoization?
Recursive Fibonacci is super slow without memoization because it re-computes the same values over and over.

Memoization means:

We store results of previous calls

If we already solved fib(n), we return it from cache
*/

use std::collections:HashMap;

fn fibonacci_memoization(n:u64, memo: &mut HashMap<u64,u64>) -> u64 {
    if n == 0 {
        return 0;

    }else if n == 1 {
        return 1;
    }

    if let Some(&val) = memo.get(&n){
        return val;
    }

    let result = fibonacci_memoization(n-1, memo) + fibonacci_memoization(n-2, memo);
    memo.insert(n, result);
    result

}

fn main() {
    let n = 10;

    let mut memo = HashMap::new();
    let result = fib_memo(n, &mut memo);

    println!("The {}th Fibonacci number is {}", n, result);
}

