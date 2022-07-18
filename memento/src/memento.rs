pub struct Originator {
    state: String,
}

impl Originator {
    pub fn new() -> Self {
        Originator {
            state: String::new(),
        }
    }

    pub fn state(&self) -> &str {
        &self.state
    }

    pub fn set_state(&mut self, state: &str) {
        self.state = state.to_owned();
    }

    pub fn save(&self) -> Memento {
        Memento {
            state: self.state.clone(),
        }
    }

    pub fn restore(&mut self, memento: Memento) {
        self.state = memento.state;
    }
}

pub struct Memento {
    state: String,
}

pub struct Caretaker {
    mementos: Vec<Memento>,
}

impl Caretaker {
    pub fn new() -> Self {
        Caretaker { mementos: vec![] }
    }

    pub fn backup(&mut self, memento: Memento) {
        self.mementos.push(memento);
    }

    pub fn undo(&mut self) -> Result<Memento, &'static str> {
        match self.mementos.pop() {
            Some(memento) => Ok(memento),
            None => Err("nothing is saved"),
        }
    }

    pub fn show_history(&self) {
        for memento in self.mementos.iter().rev() {
            println!("{}", memento.state);
        }
    }
}
