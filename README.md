# Linked-List

This assignment will test your Rust knowledge by requiring you to build a [LinkedList](https://en.wikipedia.org/wiki/Linked_list).

Author(s):

- Soham Dongargaonkar
- Gagan Hegde

## Background

Linked lists are a trivial data structure to implement in most languages. However, as you will soon learn, implementing a linked list in Rust is far from trivial - in fact, doing so can get so complicated and involved that there's an [unofficial mini-book](https://rust-unofficial.github.io/too-many-lists/) on the subject.

In most scenarios, you should stick to avoiding linked lists at all. It has been shown that array backed lists [are much faster than linked lists](https://www.youtube.com/watch?v=YQs6IC-vgmo). The added complexity and unmaintanability of the code don't help, either.

In this assignment, you will learn exactly why. You will be forced to understand Rust's philosophies by doing the exact things Rust was designed to not do.

## Preliminaries

Before starting this assignment, make sure you have at least a basic understanding of:
- Ownership
- Borrow rules
- Lifetimes
- Reference Counting
- RefCells

Additionally, you are even encouraged to (but not required to) read Chapters 1-5 the [mini-book](https://rust-unofficial.github.io/too-many-lists/). 

## Your work
 - Implement the the methods that are marked by `unimplemented!()`. You can test each method by running `cargo test method_name`.
 - Edit `q_and_a.md` to answer the questions listed in it.
 - As a challenge, you should try to implement `find` so that instead of returning a `bool`, it returns a mutable reference to the value in the linked list.

## Known Issues & Limitations

The initial idea was to have the students make the `LinkedList` and create a `HashMap` (using chaining) that uses the same list. However, as time passed, we removed the `HashMap` requirement. Mainly, this was because:
- we couldn't come up with an implementation for `find` that returns a mutable reference to the value inside. This is required for the hash map to work (if a key is already present, the mutable reference could then upate the value corresponding to it). We do believe such an implementation should be possible; after all it IS possible to return a mutable reference to the head of the tail of the linked list, so it should also be possible to return a mutable reference to any node inside the list.
- even if such an implementation of `find` was possible, we agreed that the extra time students spent implementing the hash map itself would likely not use any Rust specific knowledge; as such it would simply a matter of coding until tests passed.
- Most importantly, we realized that reading the 5 chapters from the book should require a student to spend enough time on an assignment. We really wanted to add more questions to `q_and_a.md`, but we couldn't find time to dig through the book again and come up with more questions.
- Unfortunately, we do realize that the book can cause students to not have to think for themselves; and simply follow the book's instructions to get a working implementation of a list. However, we belive the book is so well written that even such an exercise would be fruitful; for example, both of us felt that our understanding of `Rc`s and `RefCell`s was much better than before after designing this assignment (which in turn required reading the book). The questions in `q_and_a.md` should ensure that a student has really read the book and not blindly used the code from it;  however both of us agree that it needs more questions to really ensure that.
