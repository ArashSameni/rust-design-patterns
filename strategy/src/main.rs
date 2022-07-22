use strategy::{AddStrategy, Context, MultiplyStratrgy, Point};

mod strategy;

fn main() {
    let mut context = Context::new(Point(5, 10), Box::new(AddStrategy));
    println!("AddStrategy: {}", context.execute_strategy());

    context.set_strategy(Box::new(MultiplyStratrgy));
    println!("MultiplyStratrgy: {}", context.execute_strategy());
}
