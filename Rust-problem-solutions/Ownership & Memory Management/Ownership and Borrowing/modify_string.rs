// 39. Implement a function that modifies a string in-place vs creating a new one.

fn modify_in_place(s: &mut String){
    s.push_str(" world")
}

fn create_new_String(s: &str) -> String{
    format("{} World", s)
}

fn main(){
    let mut orignal = String::from("hello");
    modify_in_place(&mut orignal);
    println!("orignal string: {}", orignal);

    let orignal = String::from("hello");
    let new_string = create_new_String(&orignal);
    println!("new string: {}", new_string);
}