//16. Implement a `Car` struct with fuel consumption tracking and maintenance scheduling.

struct Car{
    make: String,
    model:String,
    year: u16,
    fuel_level:f64,
    service_interval:u16,
    last_service_date:u16,



impl car{
    fn new(make:String, model:String, year:u16)-> Self{
        Self{
            make,
            model,
            year,
            fuel_level: 100.0,
            service_interval: 10000,
            last_service_date:year,

        }

        fn drive(&mut self, distance:f64){
            let fuel_consumption = distance*0.1;
            if fuel_consumption > self.fuel_level{
                println!("Not enough fuel to drive {} km", distance);
                return;
            }
        }

        fn refuel(&mut self, amount:f64){
            self.fuel_level = (self.fuel_level + amount).min(100.0);
            println!("Refueled {} liters. Current fuel level: {}%", amount, self.fuel_level);

        }
        fn schedule_service(&mut self){
            let current_year = 2025;
            let service_due = self.last_service_date + self.service_interval;
            if current_year >= service_due{
                println!("Service due for {} {} ({})", self.make, self.model, self.year);
            } else{
                println!("Service not due for {} {} ({})", self.make, self.model, self.year);
            }
        }
    }
}
fn main(){
    let mut car = Car::new(String::from("Toyota"), String::from("Corolla"), 2020);
    car.drive(100.0);
    car.refuel(20.0);
    car.schedule_service();
}
}