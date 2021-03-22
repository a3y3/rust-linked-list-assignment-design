use std::cell::Ref;
use std::cell::RefCell;
use std::rc::Rc;

struct Node<T> {
    val: T,
    next: Option<Rc<RefCell<Node<T>>>>,
    prev: Option<Rc<RefCell<Node<T>>>>,
}

impl<T> Node<T> {
    fn new(val: T) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Node {
            val,
            next: None,
            prev: None,
        }))
    }
}

pub struct LinkedList<T> {
    head: Option<Rc<RefCell<Node<T>>>>,
    tail: Option<Rc<RefCell<Node<T>>>>,
    size: usize,
}

impl<T> LinkedList<T> {
    pub fn new() -> LinkedList<T> {
        let head = None;
        let tail = None;
        let size = 0;
        LinkedList { head, tail, size }
    }

    /// Returns the number of elements in the list.
    /// This function runs in `O(1)` time.
    pub fn size(&self) -> usize {
        self.size
    }

    /// Add `value` to the start of the list.
    /// This function runs in `O(1)` time.
    pub fn push_front(&mut self, value: T) {
        let node: Rc<RefCell<Node<T>>> = Node::new(value);
        match self.head.take() {
            None => {
                self.head = Some(Rc::clone(&node));
                self.tail = Some(node);
            }
            Some(old_head) => {
                old_head.borrow_mut().prev = Some(Rc::clone(&node));
                node.borrow_mut().next = Some(Rc::clone(&old_head));
                self.head = Some(node);
            }
        }

        self.size += 1;
    }

    /// Add `value` to the end of the list.
    /// This function runs in `O(1)` time.
    pub fn push_back(&mut self, value: T) {
        unimplemented!()
    }

    /// Returns a reference to the first value of the list.
    /// This function runs in `O(1)` time.
    pub fn peek_front(&self) -> Option<Ref<T>> {
        self.head.as_ref().map(|old_head| Ref::map(old_head.borrow(), |v| &v.val))
    }

    /// Returns a reference to the last value of the list.
    /// This function runs in `O(1)` time.
    pub fn peek_back(&self) -> Option<&T> {
        unimplemented!()
    }

    /// Removes the first element from the list and return it
    /// This function runs in `O(1)` time.
    pub fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|old_head| {
            match old_head.borrow_mut().next.take() {
                None => self.tail = None,
                Some(v) => {
                    v.borrow_mut().prev = None;
                    self.head = Some(v);
                }
            };
            self.size -= 1;
            Rc::try_unwrap(old_head).ok().unwrap().into_inner().val
        })
    }

    /// Removes the last element from the list and return it
    /// This function runs in `O(1)` time.
    pub fn pop_back(&mut self) -> Option<T> {
        unimplemented!()
    }

    /// Finds if a value is present in the list.
    /// If the value is not found, return None
    /// If the value is found, return a mutable reference to the value.
    pub fn find(&mut self, value: &T) -> Option<&mut T> {
        unimplemented!()
    }

    /// Removes the value from the linkedlist.
    /// If the value was present, return that value, else return None.
    pub fn remove(&mut self, value: &T) -> Option<T> {
        unimplemented!()
    }
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        while self.pop_front().is_some() {}
    }
}

mod tests;
