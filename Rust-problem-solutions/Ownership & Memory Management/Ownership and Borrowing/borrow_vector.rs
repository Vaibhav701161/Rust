// 32. Write a function that takes a vector of integers and returns the sum of all the elements.

fn main(){
    let vec: Vec<i32> = vec![1,2,3,4,5];
    let sum = vector_sum(vec);
    println!("the sum of the elements of vector is {}", sum);
}

fn sum(vec: Vec<i32>)-> i32{

    let mut sum = 0;
    for i in vec{
        sum = sum+i;
    }
    sum;
}