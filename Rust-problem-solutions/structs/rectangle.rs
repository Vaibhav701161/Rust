// 12. Implement a `Rectangle` struct with methods for area, perimeter, and overlap detection.
struct Rectangle{
    width:f64,
    hieght:f64,
}

impl Rectangle{

    fn area(&self) -> f64{
        self.width*self.hieght
    }

    fn perimeter(&self)->f64{
       ( self.width + self.hieght)* 2.0
    }
    fn overlap(&self, other: &Rectangle) -> bool{
        self.width > other.width && self.hieght > other.hieght
        && self.width < other.width + other.hieght
        && self.hieght < other.hieght + other.width


    }

    fn main(){
        let rect1 = Rectangle{width: 10.0, hieght: 5.0};
        let rect2 = Rectangle{width: 8.0, hieght: 6.0};

        println!("Area: {}", rect1.area());
        println!("Perimeter: {}", rect1.perimeter());
        println!("Overlap: {}", rect1.overlap(&rect2));
    }
}