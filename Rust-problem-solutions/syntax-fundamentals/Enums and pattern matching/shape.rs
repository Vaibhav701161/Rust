// 23. Create a `Shape` enum with area calculation for different shapes.

#[derive(Debug)]

enum Shape{
    Circle(f64),
    Rectangle(f64,f64),
    Triangle(f64,f64),
    square(f64),
}

impl Shape{
    fn area(&self) -> f64{
        match self{
            Shape::Circle(radius) => std::f64::consts::PI*radius*radius,
            Shape::Rectangle(width,hieght) => width*hieght,
            Shape::Triangle(base,height) => 0.5*base*height,
            Shape::Square(side) => side*side,
        }
    }
}

fn main(){
    let circle = Shape::Circle(5.0);
    let rectangle = Shape::Rectangle(4.0*6.0);
    let triangle= Shape::Triangle(7.9*8.0);
    let square = Shape::Square(3.0);



    println!("Circle area: {}", circle.area());
    println!("Rectangle area: {}", rectangle.area());
    println!("Triangle area: {}", triangle.area());
    println!("Square area: {}", square.area());
}