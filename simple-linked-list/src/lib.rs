use core::marker::PhantomData;
use std::iter::FromIterator;

pub struct SimpleLinkedList<T> {
    head: Option<Box<Node<T>>>,
    len: usize,
}

struct Node<T> {
    next: Option<Box<Node<T>>>,
    element: T,
}

impl<T> Node<T> {
    fn new(next: Option<Box<Node<T>>>, element: T) -> Self {
        Self { next, element }
    }
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        Self { head: None, len: 0 }
    }

    // You may be wondering why it's necessary to have is_empty()
    // when it can easily be determined from len().
    // It's good custom to have both because len() can be expensive for some types,
    // whereas is_empty() is almost always cheap.
    // (Also ask yourself whether len() is expensive for SimpleLinkedList)
    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn push(&mut self, element: T) {
        let initial_head = self.head.take();
        self.len += 1;
        self.head = Some(Box::new(Node::new(initial_head, element)))
    }

    pub fn pop(&mut self) -> Option<T> {
        let initial_head = self.head.take();
        match initial_head {
            Some(mut head) => {
                self.head = head.next.take();
                self.len -= 1;
                return Some(head.element);
            }
            None => return None,
        }
    }

    pub fn peek(&self) -> Option<&T> {
        match &self.head {
            Some(head) => return Some(&head.element),
            None => return None,
        }
    }

    pub fn rev(self) -> SimpleLinkedList<T> {
        let mut node = self;
        let mut rev_list = SimpleLinkedList::new();

        while let Some(element) = node.pop() {
            rev_list.push(element);
        }

        rev_list
    }
}

impl<T> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut new_linked_list = SimpleLinkedList::new();
        for i in iter {
            new_linked_list.push(i)
        }
        new_linked_list
    }
}

// In general, it would be preferable to implement IntoIterator for SimpleLinkedList<T>
// instead of implementing an explicit conversion to a vector. This is because, together,
// FromIterator and IntoIterator enable conversion between arbitrary collections.
// Given that implementation, converting to a vector is trivial:
//
// let vec: Vec<_> = simple_linked_list.into_iter().collect();
//
// The reason this exercise's API includes an explicit conversion to Vec<T> instead
// of IntoIterator is that implementing that interface is fairly complicated, and
// demands more of the student than we expect at this point in the track.

impl<T> From<SimpleLinkedList<T>> for Vec<T> {
    fn from(mut linked_list: SimpleLinkedList<T>) -> Vec<T> {
        let mut new_vec = Vec::new();
        while let Some(element) = linked_list.pop() {
            new_vec.insert(0, element)
        }
        new_vec
    }
}
