// 25. Create a `State` enum for a finite state machine (e.g., traffic light).

use std::thread::sleep;
use std::time::Duration;

#[derive(Debug)]
enum State{
    Red,
    Yellow,
    Green,
}

impl State{
    fn next(&self) -> State{
        match self{
            State::Red => State::Green,
            State::Green => State::Yellow,
            State::Yellow => State::Red,

        }
    }

    fn duration(&self) -> Duration{
        match self{
            State::Red => Duration::from_secs(30),
            State::Yellow => Duration::from_secs(5),
            State::Green => Duration::from_secs(30),
        }
    }

}

fn main() {
    let mut state = State::Red;

    for _ in 0..15{
        println!("Current state: {:?}",state);
        sleep(state.duration());
        state = state.next();
    }
}