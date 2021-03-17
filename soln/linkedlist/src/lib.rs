pub struct LinkedList<T> {
    _bogus: T,
}
impl<T> LinkedList<T> {
    pub fn new(value: T) -> LinkedList<T> {
        unimplemented!()
    }

    pub fn size(&self) -> usize {
        unimplemented!()
    }

    pub fn push_front(&mut self, value: T) {
        unimplemented!()
    }

    pub fn push_back(&mut self, value: T) {
        unimplemented!()
    }

    pub fn peek_front(&self) -> &T {
        unimplemented!()
    }

    pub fn peek_back(&self) -> &T {
        unimplemented!()
    }
}

mod tests;
