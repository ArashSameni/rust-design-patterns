# Strategy

## General Info

`Strategy` is a `behavioral` design pattern that lets you define a family of algorithms, put each of them into a separate class, and make their objects interchangeable.However you can achieve this behavior with `closures`.

## Conceptual Example

Suppose you are building an In-Memory-Cache. Since it’s in memory, it has a limited size. Whenever it reaches its maximum size, some entries have to be evicted to free-up space. This can happen via several algorithms. Some of the popular algorithms are:

1. Least Recently Used (LRU)
2. First In, First Out (FIFO)
3. Least Frequently Used (LFU)

The problem is how to decouple our cache class from these algorithms so that we can change the algorithm at run time. Also, the cache class should not change when a new algorithm is being added.
This is were Strategy pattern comes into the picture.

## Applicability

Use the Strategy pattern when

* You want to use different variants of an algorithm within an object and be able to switch from one algorithm to another during runtime
* You have a lot of similar classes that only differ in the way they execute some behavior
* Your class has a massive conditional statement that switches between different variants of the same algorithm

## Resources

* [Refactoring Guru](https://refactoring.guru/)
