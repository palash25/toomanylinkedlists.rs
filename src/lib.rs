pub mod first;
pub mod fourth;
pub mod second;
pub mod third;
/*
Pop returns Option<i32>, while our implementation only manipulates Links (Box<Node>).
So our implementation only moves around pointers to nodes, while the pop-based one will move around
the values we stored in nodes. This could be very expensive if we generalize our list and someone uses
it to store instances of VeryBigThingWithADropImpl (VBTWADI). Box is able to run the drop implementation
of its contents in-place, so it doesn't suffer from this issue. Since VBTWADI is exactly the
kind of thing that actually makes using a linked-list desirable over an array, behaving poorly on
this case would be a bit of a disappointment.

If you wish to have the best of both implementations, you could add a new method,
fn pop_node(&mut self) -> Link, from-which pop and drop can both be cleanly derived.

https://rust-unofficial.github.io/too-many-lists/first-drop.html
*/
