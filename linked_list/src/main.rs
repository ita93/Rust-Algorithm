type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    value: T,
    next: Link<T>,
}

struct LinkedList<T> {
    head: Link<T>, //point to the first element
}

impl <T>Node<T> {
    fn new(value: T) -> Self{
        Self{
            value, 
            next: None,
        }
    }
}

impl <T>LinkedList<T> {
    fn new() -> Self {
        Self{
            head: None,
        }
    }

    fn add_front(&mut self, value: T) {
        let mut new_node = Node::new(value);
        new_node.next = self.head.take();
        self.head = Some(Box::new(new_node));
    }

    fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            let node = *node;
            self.head = node.next;
            node.value
        })
    }

    fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| {
            &node.value
        })
    }

    fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| {
            &mut node.value
        })
    }
    
}

fn main() {
    let mut my_list:LinkedList<u32> = LinkedList::new();

    my_list.add_front(30u32);
    println!("Head: {}", my_list.peek().unwrap());
    my_list.add_front(31u32);
    println!("Head: {}", my_list.peek().unwrap());
    my_list.add_front(32u32);
    println!("Head: {}", my_list.peek().unwrap());
}
