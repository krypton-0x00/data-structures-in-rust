#[derive(Debug)]
pub struct Stack<T> {
    items: Vec<T>,
}

impl<T> Stack<T> {
    pub fn new(data: Vec<T>) -> Self {
        Self { items: data }
    }
    pub fn push(&mut self, item: T) {
        self.items.push(item);
    }
    pub fn pop(&mut self) -> T {
        self.items.pop().unwrap()
    }
    pub fn peak(&self) -> Option<&T> {
        self.items.last()
    }
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
    pub fn size(&self) -> usize {
        self.items.len()
    }
}
