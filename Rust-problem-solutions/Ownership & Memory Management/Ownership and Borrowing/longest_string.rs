// 35. Write a function that takes two strings and returns the longest string.

fn main(){
    let string:Vec<String> = vec![
        String::from("vaibhav"),
        String::from("aditi"),
        String::from("kavish"),
    ];

    let longest = longest_string(&string);
    println!("the longest string is {}", longest);
}

fn longest_string(v: &Vec<String>) -> &String{
    let mut longest = &v[0];

    for i in v{
        if i.len() > longest.len(){
            longest = i;
        }
    }
    longest;
}