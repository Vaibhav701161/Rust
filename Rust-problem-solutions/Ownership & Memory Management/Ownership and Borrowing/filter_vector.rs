// 38. Write a function that takes a vector of integers and returns a new vector with only the even numbers.

fn main(){
    let numbers = vec![1,2,3,4,5,6,7,8,9,10];
    let even_numbers = filter_even_numbers(&numbers);
    println!("even numbers: {:?}", even_numbers);
}

fn filter_even_numbers(v: &Vec<i32> ) -> Vec<i32> {
    let mut even_numbers = Vec::new();
    for i in v{
        if i % 2 ==0{
            even_numbers.push(i);
        }
    }
    even_numbers;
}