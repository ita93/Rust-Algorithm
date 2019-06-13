type Link<T> = Option<Box<T>>;

struct Node<T> {
    value: T,
    next: Link<T>,
}

struct LinkedList<T> {
    head: Link<T>,
}

impl <T>LinkedList<T> {
    fn new() -> Self {
        Self{
            head: None,
        }
    }

    
}

fn main() {
    println!("Hello, world!");
}
