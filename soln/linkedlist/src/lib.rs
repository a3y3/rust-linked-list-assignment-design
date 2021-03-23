use std::cell::RefCell;
use std::cell::{Ref};
use std::rc::Rc;

struct Node<T>
where
    T: PartialEq + Eq,
{
    val: T,
    next: Option<Rc<RefCell<Node<T>>>>,
    prev: Option<Rc<RefCell<Node<T>>>>,
}

impl<T> Node<T>
where
    T: PartialEq + Eq,
{
    fn new(val: T) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Node {
            val,
            next: None,
            prev: None,
        }))
    }
}

pub struct LinkedList<T>
where
    T: PartialEq + Eq,
{
    head: Option<Rc<RefCell<Node<T>>>>,
    tail: Option<Rc<RefCell<Node<T>>>>,
    size: usize,
}

impl<T> LinkedList<T>
where
    T: PartialEq + Eq,
{
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
        let node: Rc<RefCell<Node<T>>> = Node::new(value);
        match self.tail.take() {
            None => {
                self.head = Some(Rc::clone(&node));
                self.tail = Some(node);
            }
            Some(old_tail) => {
                old_tail.borrow_mut().next = Some(Rc::clone(&node));
                node.borrow_mut().prev = Some(Rc::clone(&old_tail));
                self.tail = Some(node);
            }
        }
        self.size += 1;
    }

    /// Returns a reference to the first value of the list.
    /// This function runs in `O(1)` time.
    pub fn peek_front(&self) -> Option<Ref<T>> {
        self.head
            .as_ref()
            .map(|old_head| Ref::map(old_head.borrow(), |v| &v.val))
    }

    /// Returns a reference to the last value of the list.
    /// This function runs in `O(1)` time.
    pub fn peek_back(&self) -> Option<Ref<T>> {
        self.tail
            .as_ref()
            .map(|old_tail| Ref::map(old_tail.borrow(), |v| &v.val))
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
        self.tail.take().map(|old_tail| {
            match old_tail.borrow_mut().prev.take() {
                None => self.head = None,
                Some(v) => {
                    v.borrow_mut().next = None;
                    self.tail = Some(v);
                }
            };
            self.size -= 1;
            Rc::try_unwrap(old_tail).ok().unwrap().into_inner().val
        })
    }

    /// Finds if a value is present in the list.
    /// If the value is not found, return false
    /// If the value is found, return true.
    ///
    /// Challenge: Implement a version of this method that, instead of only returning `bool`,
    /// returns a mutable reference to the value.
    /// To accept this challenge, change the function return type to either:
    /// 1. Option<&mut T>
    /// 2. Option<RefMut<T>>
    ///
    pub fn find(&mut self, value: &T) -> bool {
        let mut list_cache = LinkedList::new();
        let mut current = self.pop_front();
        loop {
            match current {
                None => {
                    while list_cache.size() != 0 {
                        self.push_front(list_cache.pop_front().unwrap());
                    }
                    return false;
                }
                Some(v) => {
                    if &v == value {
                        list_cache.push_front(v);
                        while list_cache.size() != 0 {
                            self.push_front(list_cache.pop_front().unwrap());
                        }
                        return true;
                    }

                    current = self.pop_front();
                    list_cache.push_front(v);
                }
            }
        }
    }

    /// Removes the value from the linkedlist.
    /// If the value was present, return that value, else return None.
    pub fn remove(&mut self, value: &T) -> Option<T> {
        let mut list_cache = LinkedList::new();
        let mut current = self.pop_front();
        loop {
            match current {
                None => {
                    while list_cache.size() != 0 {
                        self.push_front(list_cache.pop_front().unwrap());
                    }
                    return None;
                }
                Some(v) => {
                    if &v == value {
                        while list_cache.size() != 0 {
                            self.push_front(list_cache.pop_front().unwrap());
                        }
                        return Some(v);
                    }

                    current = self.pop_front();
                    list_cache.push_front(v);
                }
            }
        }
    }
}

impl<T> Drop for LinkedList<T>
where
    T: PartialEq + Eq,
{
    fn drop(&mut self) {
        while self.pop_front().is_some() {}
    }
}

mod tests;
