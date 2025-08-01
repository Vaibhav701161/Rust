/*
fizzbuzz rules:
- for numbers divisible by 3 , print "fizz"
- for numbers divisible by 5 , print "buzz"
- for numbers divisible by both 3 and 5 , print "fizzbuzz"
- for all other numbers, print the number itself

*/

// using for loop

fn fizzbuzz_for(){
    for i 1..100 {
        if i % 3 ==0 && i % 5 == 0 {
            println!("fizzbuzz");
        
        } else if i % 3 ==0 {
            println!("fizz");
        }else if i % 5  == 0 {
            println!("buzz");

        } else {
            println!("{}", i);
        }
    }
}

// using while loop 

fn fizzbuzz_while(){
    let mut i =1;
    while i<=100 {
        if i % 3 ==0 && i % 5 ==0 {
            println!("fizzbuzz");
        }
        else if i %3 ==0 {
            println!("fizz");
        } else if i % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", i);
        }
        i += 1;
    }
}

fn fizzbuzz_loop() {
    let mut i = 1;

    loop {
        if i > 100 {
            break;
        }

        if i % 15 == 0 {
            println!("FizzBuzz");
        } else if i % 3 == 0 {
            println!("Fizz");
        } else if i % 5 == 0 {
            println!("Buzz");
        } else {
            println!("{}", i);
        }

        i += 1;
    }
}
fn main() {
    println!("Using for loop:");
    fizzbuzz_for();

    println!("\nUsing while loop:");
    fizzbuzz_while();

    println!("\nUsing loop:");
    fizzbuzz_loop();
}