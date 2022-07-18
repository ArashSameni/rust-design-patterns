use memento::{Caretaker, Originator};

mod memento;

fn main() {
    let mut originator = Originator::new();
    let mut caretaker = Caretaker::new();

    originator.set_state("A");
    caretaker.backup(originator.save());
    println!("Current State: {}", originator.state());

    originator.set_state("B");
    caretaker.backup(originator.save());
    println!("Current State: {}", originator.state());

    originator.set_state("C");
    caretaker.backup(originator.save());
    println!("Current State: {}", originator.state());

    originator.set_state("D");
    println!("Current State: {}", originator.state());

    println!("History: ");
    caretaker.show_history();

    println!("Let's rollback!");
    originator.restore(caretaker.undo().unwrap());
    println!("Current State: {}", originator.state());

    originator.restore(caretaker.undo().unwrap());
    println!("Current State: {}", originator.state());

    originator.restore(caretaker.undo().unwrap());
    println!("Current State: {}", originator.state());
}
