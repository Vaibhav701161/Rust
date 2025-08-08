// 37. Write a function that takes two variables and swaps their values.
fn main(){
    let mut a = 10;
    let mut b = 20;

    println!("a & b before swap : {} & {}", a,b);

    swap(&mut a, &mut b);
    println!("a & b after swap: {} & {}",a,b);
}

fn swap(x: &mut i32 , y: &mut i32){
    (*x, *y) = (*y, *x); //  this is called tuple assignment (Tuple Destructuring)
}