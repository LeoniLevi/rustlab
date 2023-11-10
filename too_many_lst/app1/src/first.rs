pub struct List {
    head: Link,
}

enum Link {
    Empty,
    More(Box<Node>),
}

struct Node {
    elem: i32,
    next: Link,
}

impl List {
    pub fn new() -> Self {
        List { head: Link::Empty }
    }
    pub fn push(&mut self, val: i32) {
        let node = Box::new(Node {elem: val, next: std::mem::replace(&mut self.head, Link::Empty)});
        self.head = Link::More(node);
    }

    pub fn pop(&mut self) -> Option<i32> {
        match std::mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => None,
            Link::More(node) => {
                self.head = node.next;
                Some(node.elem)
            }
        }
    }
}

impl Drop for List {
    fn drop(&mut self) {
        println!("Dropping List started");
        let mut cur_link = std::mem::replace(&mut self.head, Link::Empty);

        while let Link::More(mut node) = cur_link {
            println!("Dropping node...");
            cur_link = std::mem::replace(&mut node.next, Link::Empty);
        }

        println!("Dropping List completed");        
    }
}
