//22. Implement a `Direction` enum for a 2D game with movement logic.

#[derive(Debug)]

enum Direction{
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]

struct Position{
    x:i32,
    y:i32,
}

impl Position{
    fn move_in_direction(&mut self, direction: Direction){
        match direction{
            Direction::Up => self.y += 1;
            Direction::Down => self.y -=1;
            Direction::Left => self.x -= 1;
            Direction::Right => self.x +=1;
        }
    }
}

fn main(){
    let mut player  = Position {x:0,y:0};
    println!("Initial Position: {:?}",player);

    player.move_in_direction(Direction::Up);
    println!("After moving Up: {:?}",player);

    player.move_in_direction(Direction::Right);
    println!("After moving Right: {:?}", player);

    player.move_in_direction(Direction::Down);
    println!("After moving down: {:?}", player);


}