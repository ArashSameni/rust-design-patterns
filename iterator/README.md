# Iterator

## General Info

`Iterator` is a `behavioral` design pattern that lets you traverse elements of a collection without exposing its underlying representation (list, stack, tree, etc.).

## Conceptual Example

The main idea behind the Iterator pattern is to extract the iteration logic of a collection into a different object called iterator. This iterator provides a generic method of iterating over a collection independent of its type.

## Applicability

Use the Iterator pattern when

* To access an aggregate object's contents without exposing its internal representation.
* To support multiple traversals of aggregate objects.
* When your collection has a complex data structure under the hood, but you want to hide its complexity from clients
* When you want your code to be able to traverse different data structures or when types of these structures are unknown beforehand.

## Resources

* [Refactoring Guru](https://refactoring.guru/)
