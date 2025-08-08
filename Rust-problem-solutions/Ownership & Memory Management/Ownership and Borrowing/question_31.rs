// 31. Write a function that takes ownership of a String and returns its length.
fn main() {
    let s1: String = String::from("Hello");
    let len = calculate_length(s1);
    println!("the length of {} is {}" ,s1 , len);
}

fn calculate_length(s:String) -> usize{
    s.len();
}