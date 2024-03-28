struct ListNode<T> {
    value: T,
    next: Option<Box<ListNode<T>>>,
}

struct LinkedList<T> {
    head: Option<Box<ListNode<T>>>,
}

impl<T> LinkedList<T> {
    fn new() -> Self {
        LinkedList { head: Option::None }
    }

    fn push_left(&mut self, val: T) {
        let new_node = Box::new(ListNode {
            value: val,
            next: self.head.take(), // Takes the value out of the option, leaving a None in its place.
        });

        self.head = Some(new_node)
    }

    fn pop_left(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.value
        })
    }

    fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.value)
    }

    fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| &mut node.value)
    }

    fn into_iter(self) -> ListIntoIter<T> {
        ListIntoIter(self)
    }

    fn iter(&self) -> ListIter<T> {
        ListIter {
            next: self.head.as_deref(),
        }
    }
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        let mut cur_link = self.head.take();
        while let Some(mut boxed_node) = cur_link {
            cur_link = boxed_node.next.take();
        }
    }
}

struct ListIntoIter<T>(LinkedList<T>);

impl<T> Iterator for ListIntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop_left()
    }
}

struct ListIter<'a, T> {
    next: Option<&'a ListNode<T>>,
}

impl<'a, T> Iterator for ListIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> { 
        self.next.map(|node| { // The closure captures a copy of the node
            self.next = node.next.as_deref();
            &node.value
        })
    }
}

/*
    iter() iterates over the items by reference
    iter_mut() iterates over the items, giving a mutable reference to each item
    into_iter() iterates over the items, transfers ownership of the items to whoever uses the iterator.

    If you just need to look at the data, use iter, if you need to edit/mutate it, use iter_mut, and if you need to give it a new owner, use into_iter.
*/

fn main() {
    let mut list = LinkedList::new();
    list.push_left(32_i32);
    list.push_left(1_i32);
    list.push_left(29_i32);

    for item in list.iter() {
        println!("{}", item);
    }
}
