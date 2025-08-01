fn gcd(mut a:u32 , mut b:u32) - > u32 {
    while b!=0 {
        let temp = b;
        b = a%b;
        a = temp;
    }
    a
}

fn main (){
println!("GCD of 48 and 18 is {}", gcd(48, 18));
}