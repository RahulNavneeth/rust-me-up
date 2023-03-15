#[derive(Debug)]
pub struct Node<T>(Option<(T, Box<Node<T>>)>);

impl<T> Node<T> {
    pub fn new() -> Self {
        Node(None)
    }

    pub fn push_front(&mut self, value: T){
        let t = self.0.take();
        self.0 = Some((value , Box::new(Node(t))));
    }

    pub fn push_back(&mut self, value : T){
        match self.0{
            Some((_, ref mut child)) => child.push_back(value),
            None => self.push_front(value),
        }
    }
}

fn main() {
    let mut ll = Node::new();
    ll.push_back(1);
    ll.push_front(2);
    ll.push_back(3);
    ll.push_front(4);

    println!("{:?}", &ll);
}
