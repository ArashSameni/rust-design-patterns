# Memento

## General Info

`Memento` is a `behavioral` design pattern that lets you save and restore the previous state of an object without revealing the details of its implementation.  

The Memento doesn’t compromise the internal structure of the object it works with, as well as data kept inside the snapshots.

## Conceptual Example

The Memento pattern lets us save snapshots for an object’s state. You can use these snapshots to revert the object to the previous state. It’s handy when you need to implement undo-redo operations on an object.

## Applicability

Use the Memento pattern when

* A snapshot of an object's state must be saved so that it can be restored to that state later
* A direct interface to obtaining the state would expose implementation details and break the object's encapsulation

## Resources

* [Refactoring Guru](https://refactoring.guru/)
