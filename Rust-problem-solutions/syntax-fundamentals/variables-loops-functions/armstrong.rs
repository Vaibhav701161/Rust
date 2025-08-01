fn is_armstrong(n: u32) -> bool {
    let digits: Vec<u32> = n.to_string()
                            .chars()
                            .map(|c| c.to_digit(10).unwrap())
                            .collect();
    let power = digits.len() as u32;
    let sum: u32 = digits.iter().map(|d| d.pow(power)).sum();

    sum == n
}

fn armstrong_in_range(start: u32, end: u32) {
    for i in start..=end {
        if is_armstrong(i) {
            println!("{}", i);
        }
    }
}

fn main() {
    armstrong_in_range(1, 1000);
}
