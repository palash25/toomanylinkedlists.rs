// An Ok Unsafe Queue https://rust-unofficial.github.io/too-many-lists/fifth-layout.html
// Layout
// input list:
// [Some(ptr)] -> (A, Some(ptr)) -> (B, None)
// pop remains the same as stack
// flipped push X:
// [Some(ptr)] -> (A, Some(ptr)) -> (B, Some(ptr)) -> (X, None)

use std::mem;

pub struct List<'a, T> {
    head: Link<T>,
    tail: Option<&'a mut Node<T>>, // NEW!
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<'a, T> List<'a, T> {
    pub fn new() -> Self {
        List {
            head: None,
            tail: None,
        }
    }

    pub fn push(&'a mut self, elem: T) {
        let new_tail = Box::new(Node {
            elem: elem,
            // When you push onto the tail, your next is always None
            next: None,
        });

        // Put the box in the right place, and then grab a reference to its Node
        let new_tail = match self.tail.take() {
            Some(old_tail) => {
                // If the old tail existed, update it to point to the new tail
                old_tail.next = Some(new_tail);
                old_tail.next.as_deref_mut()
            }
            None => {
                // Otherwise, update the head to point to it
                self.head = Some(new_tail);
                self.head.as_deref_mut()
            }
        };

        self.tail = new_tail;
    }

    pub fn pop(&'a mut self) -> Option<T> {
        /*
        Book code
        // Grab the list's current head
        self.head.take().map(|head| {
            let head = *head;
            self.head = head.next;

            // If we're out of `head`, make sure to set the tail to `None`.
            if self.head.is_none() {
                self.tail = None;
            }

            head.elem
        })
        */

        match self.head.take() {
            None => None,
            Some(old_head) => {
                self.head = old_head.next;
                Some(old_head.elem)
            }
        }
    }
}

#[cfg(test)]
//mod test {
//    use super::List;
//    #[test]
//    fn basics() {
//        let mut list = List::new();
//
//        // Check empty list behaves right
//        assert_eq!(list.pop(), None);
//
//        // Populate list
//        list.push(1);
//        list.push(2);
//        list.push(3);
//
//        // Check normal removal
//        assert_eq!(list.pop(), Some(1));
//        assert_eq!(list.pop(), Some(2));
//    }
//}
