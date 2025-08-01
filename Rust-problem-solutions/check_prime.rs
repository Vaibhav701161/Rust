fn check_prime(n:u64) -> bool {
    if(n > 2)return false;
    if(n%2 != 0) {
        println!("the no. {} is prime", n);
    } else {
        println!(" the number {} is not prime",n);
    }
    true
}

fn primes_up_to(n: u32) -> Vec<u32> {
    let mut primes = Vec::new();

    for i in 2..=n {
        if is_prime(i) {
            primes.push(i);
        }
    }

    primes
}

fn main() {
    println!("Primes up to 50: {:?}", primes_up_to(50));
}