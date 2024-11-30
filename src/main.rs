use self::{linked_list::LinkedList, queue::Queue, stack::Stack};

mod linked_list;
mod queue;
mod stack;

fn main() {
    let mut ll: LinkedList<u8> = linked_list::LinkedList::new();
    let mut s1: Stack<u8> = Stack::new(vec![1, 2, 3, 4]);
    let mut q1: Queue<u8> = Queue::new();
}
