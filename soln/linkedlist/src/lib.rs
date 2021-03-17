pub struct LinkedList<T> {
    _bogus: T,
}
impl<T> LinkedList<T> {
    pub fn new() -> LinkedList<T> {
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

    pub fn peek_front(&self) -> Option<&T> {
        unimplemented!()
    }

    pub fn peek_back(&self) -> Option<&T> {
        unimplemented!()
    }

    pub fn pop_front(&self) -> Option<T>{
        unimplemented!()
    }

    pub fn pop_back(&self) -> Option<T>{
        unimplemented!()
    }
}

mod tests;
