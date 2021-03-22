# Project Q&A
1. If your implementation uses `Option<Rc<RefCell<Node<T>>>>`,
explain why you used:
    1. `Option<>`
    1. `Rc<>`
    1. `RefCell`
1. In methods `push_back()` and `push_front()`, you probably used the method `Option::take()`. Note that `take()` is just sugar for `std::mem::replace`. Considering both of these are unsafe methods, why is either of `take()` or `std::mem::replace` necessary? Is it possible to write an implementation of insertion without using these methods?

1. In methods `pop_back()` and `pop_front()`, you probably used the method `Rc::try_unwrap()`. `Rc::try_unwrap` can return an `Error`. Did you handle this error? If yes, why? If no, why is `try_unwrap()` guaranted to always return the contained value correctly?
1. In methods `peek_back()` and `peek_front()`, why doesn't the following implementation suffice:
    ```
    self.head.as_ref().map(|node| {
        &node.borrow().elem
    })
    ```
1. As an explanation for #4, the [author notes](https://rust-unofficial.github.io/too-many-lists/fourth-peek.html):
    ```
    So if we did manage to hold onto our reference longer than the Ref existed, we could get a RefMut while a reference was kicking around and totally break Rust's type system in half.
    ```
    What does the author mean? If we are returning an `Option<Ref<T>>`, which reference could be "kicking around" and which `RefMut` could we "get"?
