// 17. Create a `DateTime` struct that handles date arithmetic and formatting.

struct DateTime{
    year: u16,
    month: u8,
    day: u8,
    hour: u8,
    second: u8,
}

impl DateTime{
    fn new(year:u16, month: u8, day:u8, hour:u8, second:u8)-> Self{
        Self{
            year,
            month,
            day,
            hour,
            second,
        }
    }
    fn add_days(&mut self, days:u8){
        let days_in_month = [31,28,31,30,31,30,31,31,30,31,30,31];
        self.day+=days;
        while self.day > days_in_month[self.month as usize - 1]{
            self.day-= days_in_month[self.month as usize - 1];
            self.month+=1;
            if self.month > 12{
                self.month=1;
                self.year+=1;

            }

            fn format(&self)->String{
                format!("{}-{}-{} {}:{}", self.year, self.month, self.day, self.hour, self.second)
            }

            fn main(){
                let mut date_time = DateTime::new(2025, 1, 1, 12, 0);
                date_time.add_days(10);
                println!("Formatted date: {}", date_time.format());
            }
        }
    }
}