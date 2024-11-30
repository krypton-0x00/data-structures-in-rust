use std::collections::VecDeque;

#[derive(Debug)]
pub struct Queue<T> {
    items: VecDeque<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Self {
        Self {
            items: VecDeque::new(),
        }
    }
    pub fn enqueue(&mut self, item: T) {
        self.items.push_back(item)
    }
    pub fn dequeue(&mut self) -> Option<T> {
        self.items.pop_front()
    }
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
    pub fn peak(&self) -> Option<&T> {
        self.items.front()
    }
    pub fn size(&self) -> usize {
        self.items.len()
    }
}
