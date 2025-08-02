struct Person{
    first_name: String,
    last_name: String,
    birth_date: u32,
}

impl Person {
    fn full_name(&self) -> String{
        format!("{} {}", self.first_name, self.last_name);
    }

    fn age(&self) -> u32{
        if current_year() > self.birth_date{
            current_year() - self.birth_date
        } else{
            0
        }
    }

    fn is_valid(&self) -> bool{
        !self.first_name.trim().is_empty() && !self.last_name.trim().is_empty() 
        && self.birth_date > = 1900 && self.birth_date <= current_year()
    }

    fn main() {
        let person = Person{
            first_name: String::from("John"),
            last_name: String::from("Doe"),
            birth_date: 1990,
        };

        println!("{}", person.full_name());
    }

}