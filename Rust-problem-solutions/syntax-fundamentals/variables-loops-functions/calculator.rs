fn calculator(a: f64, b: f64, op: char) -> f64 {
    match op {
        '+' => a + b,
        '-' => a - b,
        '*' => a * b,
        '/' => {
            if b != 0.0 {
                a / b
            } else {
                println!("Cannot divide by zero!");
                0.0
            }
        }
        _ => {
            println!("Invalid operator!");
            0.0
        }
    }
}

fn main() {
    println!("5 + 3 = {}", calculator(5.0, 3.0, '+'));
}
