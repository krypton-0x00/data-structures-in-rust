use std::{cell::RefCell, rc::Rc};

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

#[derive(Debug)]
struct Node<T> {
    data: T,
    next: Link<T>,
}
#[derive(Debug)]
pub struct LinkedList<T> {
    head: Link<T>,
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self { head: None }
    }
    pub fn push(&mut self, value: T) {
        let new_node = Rc::new(RefCell::new(Node {
            data: value,
            next: self.head.take(), //.take() -> takes Current Head and leaves None at its place
        }));
        self.head = Some(new_node);
    }

    pub fn pop(&mut self) -> Option<T>
    where
        T: Clone,
    {
        self.head.take().map(|node| {
            let mut node = node.borrow_mut();
            let next = node.next.take();
            self.head = next;
            node.data.clone()
        })
    }
    pub fn peak(&self) -> Option<T>
    where
        T: Clone,
    {
        self.head.as_ref().map(|node| node.borrow().data.clone())
    }
    pub fn print(&self)
    where
        T: std::fmt::Display,
    {
        let mut current = self.head.clone();
        while let Some(node) = current {
            let node_ref = node.borrow();
            println!("{}", node_ref.data);
            current = node_ref.next.clone();
        }
    }
}
