// 26. Implement a `Token` enum for a simple lexer (Number, Identifier, Operator).

#[erive(Debug)]

enum Token{
    Number(i32),
    Identifier(String),
    Operator(String),
}

fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(&c) = chars.peek() {
        if ch.is_whitespce(){
            chars.next();
        } else if ch.is_digit(10){
            let mut num = String::new();
            while let Some(&c) = chars.peek(){
                if c.is_digit(10){
                    num.push(c);
                    chars.next();
                }else{
                    break;
                }
            }

            let value = num.parse::<i32>().unwrap();
            tokens.push(Token::Number(value));
        } else if ch.is_alphabetic(){
            let mut ident = String::new();
            while let Some(&c) = chars.peek() {
                if c.is_alphanumeric(){
                    ident.push(c);
                    chars.next();
                }else{
                    break;
                }

            }
            tokens.push(Token::Identifier(ident));
        } else if "+-*/".contains(c){
            tokens.push(Token::Operator(ch));
            chars.next();
        } else {
            panic!("Unexpected character: {}", c);
        }
    }
    tokens
}

fn main() {
    let input = "x = 42 + y * 2";
    let tokens = tokenize(input);

    println!("Input: {}", input);
    println!("Tokens:");
    for token in tokens {
        println!("{:?}", token);
    }
}