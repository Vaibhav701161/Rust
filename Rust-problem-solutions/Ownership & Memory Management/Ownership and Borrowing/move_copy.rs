// 34. Write a function that takes a string and returns the number of vowels in the string.

fn main(){
    let x = 10;
    let y = x;

    println!("x = {}", x);
    println!("y = {}", y);

    let s1 = String::from("Hello");
    let s2 = s1;

    println!("s1 = {}", s1); // error 
    println!("s2 = {}", s2);

    
}