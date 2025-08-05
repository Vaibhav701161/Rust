//19. Create a `Complex` number struct with arithmetic operations.
struct Complex{
    real: f64,
    imaginary: f64,
}

impl Complex{
    fn new(real: f64, imaginary: f64)-> Self{
        Self{real, imaginary}
    }

    fn add(&self, other: &Complex)-> Complex{
        Complex{
            real: self.real + other.real,
            imaginary: self.imaginary + other.imaginary,
        }
    }

    fn multiply(&self, other: &Complex)-> Complex{
        Complex{
            real: self.real * other.real - self.imaginary * other.imaginary,
            imaginary: self.real * other.imaginary + self.imaginary * other.real,
        }
    }
    

    fn main(){
        let complex1 = Complex::new(1.0, 2.0);
        let complex2 = Complex::new(3.0, 4.0);
        let result_add = complex1.add(&complex2);
        let result_multiply = complex1.multiply(&complex2);
        println!("Addition: {} + {}i", result_add.real, result_add.imaginary);
        println!("Multiplication: {} + {}i", result_multiply.real, result_multiply.imaginary);
    }
}