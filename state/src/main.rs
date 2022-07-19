use state::Context;

mod state;

fn main() {
    let mut context = Context::new();
    context.request();
    context.request();
    context.request();
    context.request();
}
