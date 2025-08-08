// 33. Write a function that takes a vector of integers and sorts them in ascending order.

fn main(){
    let mut numbers = vec![3,4,6,4,8,5,7,6];
    function_sort(&mut numbers);
    println!("sorted vector is {:?}", numbers);
}

fn function_sort(v: &mut Vec<i32>) {
    v.sort();
}