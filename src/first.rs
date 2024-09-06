use std::mem;

#[derive(Debug)]
enum Link {
    Empty,
    More(Box<Node>),
}

#[derive(Debug)]
struct Node {
    elem: i32,
    next: Link,
}

#[derive(Debug)]
pub struct List {
    head: Link,
}

impl List {
    pub fn new() -> Self {
        List { head: Link::Empty }
    }

    pub fn push(&mut self, elem: i32) {
        let n = Link::More(Box::new(Node {
            elem: elem,
            // mem::replace helps us steal a value out of a borrow in this case
            // &mut self.head and replace it with another value
            next: mem::replace(&mut self.head, Link::Empty),
        }));

        self.head = n
    }

    pub fn pop(&mut self) -> Option<i32> {
        if let Link::More(n) = mem::replace(&mut self.head, Link::Empty) {
            self.head = n.next;
            return Some(n.elem);
        }
        None
    }

    pub fn head(&self) -> Option<i32> {
        if let Link::More(n) = &self.head {
            return Some(n.elem);
        }
        None
    }
}

impl Drop for List {
    // We can't drop the contents of the Box after deallocating, so there's no way to drop in a tail-recursive manner!
    // Instead we're going to have to manually write an iterative drop for List that hoists nodes out of their boxes.
    fn drop(&mut self) {
        let mut current = mem::replace(&mut self.head, Link::Empty);

        while let Link::More(mut n) = current {
            current = mem::replace(&mut n.next, Link::Empty);
            // boxed_node goes out of scope and gets dropped here;
            // but its Node's `next` field has been set to Link::Empty
            // so no unbounded recursion occurs.

            // TODO; didn't get this
        }
    }
}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    pub fn basic() {
        let mut list = List::new();

        // Check empty list behaves right
        assert_eq!(list.pop(), None);

        // Populate list
        list.push(1);
        list.push(2);
        list.push(3);

        // Check normal removal
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));

        // Push some more just to make sure nothing's corrupted
        list.push(4);
        list.push(5);

        // Check normal removal
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));

        // Check exhaustion
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }
}
