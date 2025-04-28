# Understanding Ownership

Ownership is a set of rules that govern how a Rust program manages memory.

---

## The Stack and the Heap

### Stack

The stack an the heap are parts of memory available at your disposal at runtime, however, they are structured differently.

For example, the Stack stores values in the order it gets them and removes the values in the the opposite order. It's the classical LiFo (Last In, First Out).

Adding the dara is called pushing onto the stack and removing data is called popping off the stack.

All data to be stored on the stack must be of fixed, known sizes. The size must be known ar compile time.

### Heap

The Heap works differently from the stack. In this case, you request for a certain amount of space. The memory allocator then finds a spot that's suitable and returns a pointer.

---

## Ownership Rules

- Each value in Rust has an owner.
- There can only be one owner at a time.
- When the owner goes out of scope, the value will be dropped.
