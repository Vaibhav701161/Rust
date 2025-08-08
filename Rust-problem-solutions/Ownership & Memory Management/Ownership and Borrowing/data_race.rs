// 40. Implement a function that demonstrates a data race condition.
 /*
 A data race occurs when:

Two or more pointers access the same memory location at the same time.

At least one of them is writing.

There’s no mechanism to prevent simultaneous access (i.e., no synchronization).

In languages like C or C++, these can cause undefined behavior at runtime. But in Rust, the compiler catches such problems at compile time, thanks to its ownership and borrowing system.


 */

 fn main(){
    let mut s = String::from("hello");

    let r1 = &mut s;
    r1.push_str(" world");
    println!("r1: {}",r1);
 }