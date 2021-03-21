struct Node<T> {
    val: T,
    next: Option<Box<Node<T>>>,
    prev: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    fn new(val: T) -> Node<T> {
        Node {
            val,
            next: None,
            prev: None,
        }
    }
}

pub struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
    tail: Option<Box<Node<T>>>,
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
        let mut node = Node::new(value);
        self.head.take().map(|x| {
            node.next = Some(x);
        });
        self.head = Some(Box::new(node));
        self.size += 1;
    }

    /// Add `value` to the end of the list.
    /// This function runs in `O(1)` time.
    pub fn push_back(&mut self, value: T) {
        unimplemented!()
    }

    /// Returns a reference to the first value of the list.
    /// This function runs in `O(1)` time.
    pub fn peek_front(&self) -> Option<&T> {
        match &self.head {
            None => None,
            Some(x) => {
                let node: &Node<T> = &**x;
                Some(&node.val)
            }
        }
    }

    /// Returns a reference to the last value of the list.
    /// This function runs in `O(1)` time.
    pub fn peek_back(&self) -> Option<&T> {
        unimplemented!()
    }

    /// Removes the first element from the list and return it
    /// This function runs in `O(1)` time.
    pub fn pop_front(&mut self) -> Option<T> {
        match self.head.take() {
            None => None,
            Some(old_head) => {
                self.head = (*old_head).next;
                self.size -= 1;
                Some((*old_head).val)
            }
        }
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

mod tests;
