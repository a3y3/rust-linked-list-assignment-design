use std::cell::Ref;
struct Node<T>
where
    T: PartialEq + Eq,
{
    _bogus: T,
}

impl<T> Node<T>
where
    T: PartialEq + Eq,
{
    //Here, implement a new() function that creates a new node the way you think is appropriate.
}

pub struct LinkedList<T>
where
    T: PartialEq + Eq,
{
    _bogus: T
}

impl<T> LinkedList<T>
where
    T: PartialEq + Eq,
{
    pub fn new() -> LinkedList<T> {
        unimplemented!()
    }

    /// Returns the number of elements in the list.
    /// This function runs in `O(1)` time.
    pub fn size(&self) -> usize {
        unimplemented!()
    }

    /// Add `value` to the start of the list.
    /// This function runs in `O(1)` time.
    pub fn push_front(&mut self, value: T) {
        unimplemented!()
    }

    /// Add `value` to the end of the list.
    /// This function runs in `O(1)` time.
    pub fn push_back(&mut self, value: T) {
        unimplemented!()
    }

    /// Returns a reference to the first value of the list.
    /// This function runs in `O(1)` time.
    pub fn peek_front(&self) -> Option<Ref<T>> {
        unimplemented!()
    }

    /// Returns a reference to the last value of the list.
    /// This function runs in `O(1)` time.
    pub fn peek_back(&self) -> Option<Ref<T>> {
        unimplemented!()
    }

    /// Removes the first element from the list and return it
    /// This function runs in `O(1)` time.
    pub fn pop_front(&mut self) -> Option<T> {
        unimplemented!()
    }

    /// Removes the last element from the list and return it
    /// This function runs in `O(1)` time.
    pub fn pop_back(&mut self) -> Option<T> {
        unimplemented!()
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
        unimplemented!()
    }

    /// Removes the value from the linkedlist.
    /// If the value was present, return that value, else return None.
    pub fn remove(&mut self, value: &T) -> Option<T> {
        unimplemented!()
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
