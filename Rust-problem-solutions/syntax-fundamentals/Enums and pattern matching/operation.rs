// 30. Implement a `Operation` enum for a calculator with evaluation logic.


#[derive(Debug)]

enum Operation{
    Add,
    Substract,
    Multiply,
    Divide,
}

impl Operation{
    fn eval(&self, a: f64, b:f64) -> Result<f64, String> {
        match self{
            Operation::Add=>Ok(a+b),
            Operation::Substract=>Ok(a-b),
            Operation::Multiply=>Ok(a*b),
            Operation::Divide=> {
                if b != 0.0 {
                    Ok(a/b)
                } else {
                    Err("Division by zero error".into())
                }
            }
        }
    }
}

fn main() {
    use Operation::*;

    let ops = vec![
        Add,
        Substract,
        Multiply,
        Divide,
    ];

    let a = 10.0;
    let b = 5.0;

    for op in ops{
        print!("Evaluating {:?} on {} and {}: ", op,a,b);
        match op.eval(a,b){
            Ok(result) => println!("Result: {}", result),
            Err(e) => println!("Error: {}", e),
        }
    }

    let op = Operation::Divide;
    match op.eval(10.0,0.0) {
        Ok(res) => println!("Result of Division: {}", res),
        Err(e) => println!("Error: {}", e),
    }
}