
struct ListNode<T> {
    value: T,
    next: Option<Box<ListNode<T>>>,
}

struct LinkedList<T> {
    head: Option<Box<ListNode<T>>>,
}

impl<T> LinkedList<T> {
    fn new() -> LinkedList<T> {
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
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        let mut cur_link = self.head.take();
        while let Some(mut boxed_node) = cur_link {
            cur_link = boxed_node.next.take();
        }
    }
}

fn main() {
    let mut list = LinkedList::new();
    list.push_left(32_i32);
    list.push_left(1_i32);
    list.push_left(29_i32);

    while let Some(x) = list.pop_left() {
        println!("{x}");
    }

}
