// An Ok Unsafe Queue https://rust-unofficial.github.io/too-many-lists/fifth-layout.html
// Layout
// input list:
// [Some(ptr)] -> (A, Some(ptr)) -> (B, None)
// pop remains the same as stack
// flipped push X:
// [Some(ptr)] -> (A, Some(ptr)) -> (B, Some(ptr)) -> (X, None)

use std::mem;

// TODO: READ BORROW STACKING

pub struct List<T> {
    head: Link<T>,
    // a mutable raw pointer to Node
    tail: *mut Node<T>, // NEW!
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List {
            head: None,
            tail: std::ptr::null_mut(),
        }
    }

    pub fn push(&mut self, elem: T) {
        let mut new_tail = Box::new(Node {
            elem: elem,
            next: None,
        });

        let raw_tail: *mut _ = &mut *new_tail;

        if !self.tail.is_null() {
            unsafe {
                // since self.tail is now a raw pointer we have to manually
                // deref to set it along with operator precedence
                (*self.tail).next = Some(new_tail);
            }
        } else {
            self.head = Some(new_tail);
        }

        self.tail = raw_tail;
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|old_head| {
            self.head = old_head.next;

            // even without this if block the code will pass
            // we have to take care that tail doesn't become dangling
            if self.head.is_none() {
                self.tail = std::ptr::null_mut();
            }

            old_head.elem
        })
    }
}

#[cfg(test)]
mod test {
    use super::List;
    #[test]
    fn basics() {
        let mut list = List::new();

        // Check empty list behaves right
        assert_eq!(list.pop(), None);

        // Populate list
        list.push(1);
        list.push(2);
        list.push(3);

        // Check normal removal
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), Some(2));

        // Push some more just to make sure nothing's corrupted
        list.push(4);
        list.push(5);

        // Check normal removal
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(4));

        // Check exhaustion
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), None);

        // Check the exhaustion case fixed the pointer right
        list.push(6);
        list.push(7);

        // Check normal removal
        assert_eq!(list.pop(), Some(6));
        assert_eq!(list.pop(), Some(7));
        assert_eq!(list.pop(), None);
    }
}
