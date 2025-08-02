//14. Implement a `Point` struct with distance calculation and geometric operations.

struct Point {
    x:f64,
    y:f64,
}

impl Point{
    fn distance(&self, other: &Point) -> f64{
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dx*dx + dy*dy).sqrt()
    }

    fn midpoint(&self, other: &Point) -> Point{
        Point{
            x: (self.x + other.x) / 2.0,
            y: (self.y + other.y) /2.0,
        }
    }

    fn main(){
        let point1 = Point{x: 0.0, y: 0.0};
        let point2 = Point{x: 3.0, y: 4.0};

        println!("Distance: {}", point1.distance(&point2));
        println!("Midpoint: {:?}", point1.midpoint(&point2));
    }
}