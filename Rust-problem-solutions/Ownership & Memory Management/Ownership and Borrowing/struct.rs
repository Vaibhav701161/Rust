//36. Implement a struct that owns data and provides both mutable and immutable access.

struct DataBox{
    data: String,
}

impl DataBox{
    fn new(data: String) -> Self{
        DataBox{data:content}
    }

    fn get_data(&self) -> &String{
        &self.data
    }

    fn update_data(&mut self, new_data: String){
        self.data = new_content;
    }
}

fn main(){
    let mut box1 = DataBox::new(String::from("Hello"));
    let data = box1.get_data();
    println!("data: {}", data);

    let mut box2 = DataBox::new(String::from("World"));
    let data2 = box2.get_data();
    println!("data2: {}", data2);
    
    
}