//15. Create a `Student` struct with grade management and GPA calculation.

struct Student{
    name: String,
    grades:Vec<f64>,
}

impl Student{
    fn add_grade(&mut self, grade:f64){
        self.grades.push(grade);
    }

    fn gpa(&self) -> f64{
        if self.grades.is_empty(){
            return 0.0;
        }
        let sum = self.grades.iter().sum::<f64>();
        sum / self.grades.len() as f64
    }

    fn main(){
        let mut student = Student{
            name: String::from("John"),
            grades: Vec::new(),
        };
    }
}