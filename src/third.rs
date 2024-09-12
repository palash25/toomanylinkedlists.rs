// Persistent (immutable) singly linked list
// https://rust-unofficial.github.io/too-many-lists/third-layout.html

// Common operations
// list1 = A -> B -> C -> D
// list2 = tail(list1) = B -> C -> D
// list3 = push(list2, X) = X -> B -> C -> D

// In memory repr
// list1 -> A ---+
//               |
//               v
// list2 ------> B -> C -> D
//               ^
//               |
// list3 -> X ---+

use std::sync::Arc;

type Link<T> = Option<Arc<Node<T>>>;

pub struct Node<T> {
    elem: T,
    next: Link<T>,
}

pub struct List<T> {
    head: Link<T>,
}

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn iter(&self) -> Iter<T> {
        Iter {
            next: self.head.as_deref(),
        }
    }

    pub fn prepend(&self, elem: T) -> List<T> {
        List {
            head: Some(Arc::new(Node {
                elem: elem,
                next: self.head.clone(),
            })),
        }
    }

    pub fn tail(&self) -> List<T> {
        List {
            // when to use and_then vs map????????
            head: self.head.as_ref().and_then(|node| node.next.clone()),
        }
    }

    pub fn head(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.elem)
    }
}

// Why can't we have IntoIter for this List????????

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref();
            &node.elem
        })
    }
}

// TODO: re-read Drop for both mutable and immutable lists
// https://rust-unofficial.github.io/too-many-lists/third-drop.html
impl<T> Drop for List<T> {
    fn drop(&mut self) {
        // in case of Rc's since it allows shared references through clone
        // we can't pull the node out of the Rc like we did in the case of Box
        let mut head = self.head.take();
        while let Some(node) = head {
            if let Ok(mut node) = Arc::try_unwrap(node) {
                head = node.next.take();
            } else {
                break;
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn basics() {
        let list = List::new();
        assert_eq!(list.head(), None);

        let list = list.prepend(1).prepend(2).prepend(3);
        assert_eq!(list.head(), Some(&3));

        let list = list.tail();
        assert_eq!(list.head(), Some(&2));

        let list = list.tail();
        assert_eq!(list.head(), Some(&1));

        let list = list.tail();
        assert_eq!(list.head(), None);

        // Make sure empty tail works
        let list = list.tail();
        assert_eq!(list.head(), None);
    }

    #[test]
    fn iter() {
        let list = List::new().prepend(1).prepend(2).prepend(3);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
    }
}
