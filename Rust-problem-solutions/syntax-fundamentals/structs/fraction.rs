//20. Implement a `Fraction` struct with reduction and arithmetic operations.
struct Fraction{
    numerator: i32,
    denominator: i32,
}

impl Fraction{
    fn new(numerator: i32, denominator: i32)-> Self{
        Self{numerator, denominator}
    }

    fn reduce(&self)-> Fraction{
        let gcd = gcd(self.numerator, self.denominator);
        Fraction{
            numerator: self.numerator / gcd,
            denominator: self.denominator / gcd,
        }
    }

    fn add(&self, other: &Fraction)-> Fraction{
        let new_numerator = self.numerator * other.denominator + other.numerator * self.denominator;
        let new_denominator = self.denominator * other.denominator;
        Fraction{
            numerator: new_numerator,
            denominator: new_denominator,
        }.reduce()
    }
    
    fn multiply(&self, other: &Fraction)-> Fraction{
        Fraction{
            numerator: self.numerator * other.numerator,
            denominator: self.denominator * other.denominator,
        }.reduce()
    }
    
    fn main(){
        let fraction1 = Fraction::new(1, 2);
        let fraction2 = Fraction::new(3, 4);
        let result_add = fraction1.add(&fraction2);
        let result_multiply = fraction1.multiply(&fraction2);
        println!("Addition: {} / {}", result_add.numerator, result_add.denominator);
        println!("Multiplication: {} / {}", result_multiply.numerator, result_multiply.denominator);
    }
        
}