# State

## General Info

`State` is a `behavioral` design pattern that lets an object alter its behavior when its internal state changes.
The pattern extracts state-related behaviors into separate state classes and forces the original object to delegate the work to an instance of these classes, instead of acting on its own.

## Usage Example

The State pattern is commonly used to convert massive if-else statements into objects.

## Applicability

Use the State pattern when

* An object's behavior depends on its state, and it must change its behavior depending on that state
* When you have a lot of duplicate code across similar states and transitions of a condition-based state machine

## Resources

* [Refactoring Guru](https://refactoring.guru/)
