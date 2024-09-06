use std::mem;

type Link<T> = Option<Box<Node<T>>>;

#[derive(Debug)]
struct Node<T> {
    elem: T,
    next: Link<T>,
}

#[derive(Debug)]
pub struct List<T> {
    head: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn push(&mut self, elem: T) {
        let n = Some(Box::new(Node {
            elem: elem,
            next: mem::replace(&mut self.head, None),
        }));

        self.head = n
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.elem
        })
    }

    pub fn peek(&self) -> Option<&T> {
        // since `.map()` takes a `self` or Option<T> we needed to use head.as_ref() to change it to `&self` or Option<&T>
        // since we don't want to move the value out of self.head

        // node.elem autoderefs &BoxNode<T> to T and then we take a ref to that and return &T
        self.head.as_ref().map(|node| &node.elem)
    }
    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| &mut node.elem)
    }
    //pub fn head(&self) -> Option<T> {
    //    if let Some(n) = &self.head {
    //        return Some(n.elem);
    //    }
    //    None
    //}
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut current = self.head.take();

        while let Some(mut n) = current {
            current = n.next.take();
        }
    }
}

// There are 3 kinds of iterator traits:
// IntoIterator -> returns T
// Iter -> returns &T
// IterMut -> returns &mut T

pub struct IntoIter<T>(List<T>);

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

pub struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>,
}

impl<T> List<T> {
    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }

    pub fn iter<'a>(&'a self) -> Iter<'a, T> {
        // hold a pointer to the current node
        Iter {
            // as_deref converts Option<Box<T>> to Option<&T>
            next: self.head.as_deref(),
        }
    }

    pub fn iter_mut(&mut self) -> IterMut<T> {
        IterMut {
            next: self.head.as_deref_mut(),
        }
    }
}

// Simple returns an owned value using pop
impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    // yeilds a pointer to the node and holds onto the pointer to the next one
    fn next(&mut self) -> Option<Self::Item> {
        // Option, & all of them implement Copy
        self.next.map(|node| {
            self.next = node.next.as_deref();
            &node.elem
        })
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        // since &mut exclusive references don't implement Copy we need to use take() here
        self.next.take().map(|node| {
            self.next = node.next.as_deref_mut();
            &mut node.elem
        })
    }
}

#[cfg(test)]
mod test {
    use super::List;

    //fn make_list() -> List<i32> {}

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

    #[test]
    pub fn peek() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);

        assert_eq!(list.peek(), Some(&3));

        list.pop();
        assert_eq!(list.peek(), Some(&2));
        assert_eq!(list.peek_mut(), Some(&mut 2));

        list.peek_mut().map(|n| *n = 22);
        assert_eq!(list.peek(), Some(&22))
    }

    #[test]
    pub fn into_iter() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), None);
    }

    #[test]
    pub fn iter() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iter_mut() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.iter_mut();
        assert_eq!(iter.next(), Some(&mut 3));
        assert_eq!(iter.next(), Some(&mut 2));
        assert_eq!(iter.next(), Some(&mut 1));
    }
}
