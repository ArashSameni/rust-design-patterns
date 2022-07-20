use iterator::Container;

mod iterator;

fn main() {
    let mut container = Container::new();
    container.add_item(1);
    container.add_item(2);
    container.add_item(3);

    for i in container.iter() {
        println!("{}", i);
    }
}
