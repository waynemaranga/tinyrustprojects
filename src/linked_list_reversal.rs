// LINKED LISTS
#[derive(Debug)]
struct ListNode<T> {
    // node definition
    value: T, // check clippy warning for unused value
    next: Option<Box<ListNode<T>>>,
}

impl<T> ListNode<T> {
    // implementation with generic type
    fn new(value: T) -> Self {
        ListNode { value, next: None }
    }
}

// Function to reverse the linked list
fn reverse_list<T>(mut head: Option<Box<ListNode<T>>>) -> Option<Box<ListNode<T>>> {
    let mut prev: Option<Box<ListNode<T>>> = None;
    while let Some(mut node) = head.take() {
        let next: Option<Box<ListNode<T>>> = node.next.take();
        node.next = prev.take();
        prev = Some(node);
        head = next;
    }
    prev
}

pub fn driver() {
    let mut list: Option<Box<ListNode<i32>>> = Some(Box::new(ListNode::new(1)));
    list = Some(Box::new(ListNode {
        value: 2,
        next: list,
    }));
    list = Some(Box::new(ListNode {
        value: 3,
        next: list,
    }));

    println!("Original list: {:?}", list);

    let reversed_list = reverse_list(list);
    println!("Reversed list: {:?}", reversed_list);
}
