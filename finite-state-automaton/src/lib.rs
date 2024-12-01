#[derive(Debug, Clone, Copy)]
enum State {
    Start,
    A,
    B,
    C,
}

pub fn recognize_pattern(input: &str) -> bool {
    let mut state = State::Start;

    for ch in input.chars() {
        state = match (state, ch) {
            (State::Start, 'a') => State::A,
            (State::A, 'b') => State::B,
            (State::A, 'c') => State::C,
            (State::B, 'b') => State::B,
            (State::B, 'c') => State::C,
            _ => return false,
        }
    }

    matches!(state, State::C)
}
