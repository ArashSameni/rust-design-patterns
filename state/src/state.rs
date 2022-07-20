pub struct Context {
    state: Box<dyn State>,
}

impl Context {
    pub fn new() -> Self {
        Self {
            state: Box::new(StateA {}),
        }
    }

    pub fn request(&mut self) {
        if let Some(state) = self.state.handle() {
            self.state = state;
        }
    }
}

trait State {
    fn handle(&self) -> Option<Box<dyn State>>;
}

struct StateA {}
impl State for StateA {
    fn handle(&self) -> Option<Box<dyn State>> {
        println!("StateA handles the request");
        Some(Box::new(StateB {}))
    }
}

struct StateB {}
impl State for StateB {
    fn handle(&self) -> Option<Box<dyn State>> {
        println!("StateB handles the request");
        Some(Box::new(StateC {}))
    }
}

struct StateC {}
impl State for StateC {
    fn handle(&self) -> Option<Box<dyn State>> {
        println!("StateC handles the request");
        None
    }
}
