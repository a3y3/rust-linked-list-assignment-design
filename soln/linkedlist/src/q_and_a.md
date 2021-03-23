# Project Q&A
1. If your implementation uses `Option<Rc<RefCell<Node<T>>>>`,
explain why you used:
    1. `Option<>` - in case the next node is None. Better than using `Rc<Option>` because it prevents creation of an unnecessary node that only represents the `None` state.
    1. `Rc<>` - we need multiple pointers to a single node that must lie on the heap (if the nodes could lie on the stack, we could have gotten away with using simple references)
    1. `RefCell` - we can't only have `Rc` because we need to mutate the value inside the list by modifying the `head` and `tail` pointers (for example, while using any of the insert or delete functionalities)
1. In methods `push_back()` and `push_front()`, you probably used the method `Option::take()`. Note that `take()` is just sugar for `std::mem::replace`. Considering both of these are unsafe methods, why is either of `take()` or `std::mem::replace` necessary? Is it possible to write an implementation of insertion without using these methods?

    - It's not possible to write an implementation of insertion that doesn't use a form of `std::mem::replace`. This is because when we create a new `Node`, we need to "assign" `self.head` as the new Node's `next`. However, doing so moves the value of `self.head`, which is behind a mutable reference (and this isn't allowed). Using `take()` gives us full ownership of the value of the head while keeping a `None` in it's place as a temporary value. This value then can be assigned to the `next` of the newly created node.

1. In methods `pop_back()` and `pop_front()`, you probably used the method `Rc::try_unwrap()`. `Rc::try_unwrap` can return an `Error`. Did you handle this error? If yes, why? If no, why is `try_unwrap()` guaranted to always return the contained value correctly?
    - There isn't a need to handle this error because we use `take()` to get the old head. We are the only owners of this, so `try_unwrap()` is guaranteed to not panic.

1. In methods `peek_back()` and `peek_front()`, why doesn't the following implementation suffice:
    
    self.head.as_ref().map(|node| {
        &node.borrow().elem
    })
    
    - It's because we are returning a reference to a `Ref` instance that gets dropped when the current function ends. This could cause dangling pointers.

1. As an explanation for #4, the [author notes](https://rust-unofficial.github.io/too-many-lists/fourth-peek.html):
    
    So if we did manage to hold onto our reference longer than the Ref existed, we could get a RefMut while a reference was kicking around and totally break Rust's type system in half.
    
    What does the author mean? Which reference could be "kicking around" and which `RefMut` could we "get"?
    - We don't completely understand this statement either; honestly we believe this is a typing mistake and the author really meant "we could get a RefCell while a reference was kicking around". The reference in question is the reference that's being returned (`Option<&T>`).