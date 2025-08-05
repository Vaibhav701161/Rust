//24. Implement a `Command` enum for a simple text editor (Insert, Delete, Replace).
#[derive(Debug)]

enum Command{
    Insert(String),
    Delete(usize),
    Replace(usize, String),
}

impl Command{
    fn execute(&self, text:&mut String){
        match self{
            Command::Insert(content) => {
                text.push_str(content);

            }
            Command::Delete(index) => {
                if *index < text.len(){
                    text.remove(*index);
                }else{
                    println!("Index out of bounds for deletion.");
                }


            }

            Command::Replace(index, new_content) => {
                if *index < text.len() {
                    let mut chars: Vec<char> = text.chars().collect();
                    chars[*index] = new_content.chars().next().unwrap_or(' ');
                    *text = chars.into_iter().collect();
                }else{
                    println!("Index out of bounds for replacement.");
                }
            }
        }
    }
}

fn main() {
    let mut text = String::from("Hello,World!");
    let commands = vec![
        Command::Insert("How are you?".to_string()),
        Command::Delete(5),
        Command::Replace(7,"rust".to_string()),
    ];
}