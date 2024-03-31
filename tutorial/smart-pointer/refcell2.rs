use std::cell::RefCell;

#[derive(Debug)]
enum State {
    A,
    B,
    C,
}

struct StateMachine {
    state: RefCell<State>,
    count: RefCell<i32>,
}

impl StateMachine {
    fn new() -> StateMachine {
        StateMachine {
            state: RefCell::new(State::A),
            count: RefCell::new(0),
        }
    }

    fn next(&self) {
        let mut state = self.state.borrow_mut();
        let mut count = self.count.borrow_mut();

        match *state {
            State::A => {
                *state = State::B;
                *count += 1;
            }
            State::B => {
                *state = State::C;
                *count += 2;
            }
            State::C => {
                *state = State::A;
                *count += 3;
            }
        }
    }
}

fn main() {
    let sm = StateMachine::new();

    sm.next();
    println!(
        "State is {:?}, count is {}",
        *sm.state.borrow(),
        *sm.count.borrow()
    );

    sm.next();
    println!(
        "State is {:?}, count is {}",
        *sm.state.borrow(),
        *sm.count.borrow()
    );

    sm.next();
    println!(
        "State is {:?}, count is {}",
        *sm.state.borrow(),
        *sm.count.borrow()
    );
}
