pub struct Point(pub i32, pub i32);

pub struct Context {
    data: Point,
    strategy: Box<dyn Strategy>,
}
impl Context {
    pub fn new(data: Point, strategy: Box<dyn Strategy>) -> Self {
        Self { strategy, data }
    }

    pub fn set_strategy(&mut self, strategy: Box<dyn Strategy>) {
        self.strategy = strategy;
    }

    pub fn execute_strategy(&self) -> i32 {
        self.strategy.execute(&self.data)
    }
}

pub trait Strategy {
    fn execute(&self, data: &Point) -> i32;
}

pub struct AddStrategy;
impl Strategy for AddStrategy {
    fn execute(&self, data: &Point) -> i32 {
        data.0 + data.1
    }
}

pub struct MultiplyStratrgy;
impl Strategy for MultiplyStratrgy {
    fn execute(&self, data: &Point) -> i32 {
        data.0 * data.1
    }
}
