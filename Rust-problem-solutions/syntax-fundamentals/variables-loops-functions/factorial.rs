// iterative approach 

fn factorial_iterative(n:u64) -> u64 {
    let mut result  = 1;
     for i in 1..n{
        result = result *i;
     }
     result 
}

fn factorial_recursive(n:u64) -> u64 {
    if n == 0 {
        return 1;
    }else {
        return n*factorial_recursive(n-1);
      }
}

fn main() {
    let n = 5;
    let result = factorial_iterative(n);
    let result = factorial_recursive(n);
    println!("the factorial of {} by recursive method is {}" , n ,result);
    println!("the factorial of {} by iterative method is {}" , n ,factorial_iterative(n));
}