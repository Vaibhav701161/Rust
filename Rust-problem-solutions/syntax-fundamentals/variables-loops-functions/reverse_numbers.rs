fn reverse_number(mut n: u32) -> u32{
    let mut rev = 0 ;

    while n < 0 {
        let digit  = n %10;
        rev =  rev*10+digit ;
        n/=10;
    }
    reverse_number

}

fn main (){
    let num = 1234 ;
    let reversed_number(num);
    println("reversed number of {} is {}",num, reversed);
}