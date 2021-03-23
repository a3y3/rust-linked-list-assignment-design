pub struct LinkedList<T> {
    _bogus: T,
}
impl<T> LinkedList<T> {
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
    pub fn peek_front(&self) -> Option<&T> {
        unimplemented!()
    }

    /// Returns a reference to the last value of the list.
    /// This function runs in `O(1)` time.
    pub fn peek_back(&self) -> Option<&T> {
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
    /// If the value is not found, return None
    /// If the value is found, return a mutable reference to the value.
    pub fn find(&mut self, value: &T) -> Option<&mut T> {
        unimplemented!()
    }

    /// Removes the value from the linkedlist.
    /// If the value was present, return that value, else return None.
    pub fn remove(&mut self, value: &T) -> Option<T>{
        unimplemented!()
    }
}

mod tests;
