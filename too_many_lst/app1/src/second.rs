use std::mem;

pub struct List<T> {
    head: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }
    pub fn push(&mut self, val: T) {
        let node = Box::new(Node {
            elem: val,
            next: self.head.take(),
        });
        self.head = Some(node);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.elem
        })
    }
    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| {
            &node.elem
        })
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| {
            &mut node.elem
        })
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        println!("Dropping List started");
        let mut cur_link = mem::replace(&mut self.head, None);

        while let Some(mut node) = cur_link {
            println!("Dropping node...");
            //cur_link = mem::replace(&mut node.next, None);
            cur_link = node.next.take();
        }

        println!("Dropping List completed");
    }
}

pub struct MyIntoIter<T>(List<T>);

impl<T> List<T> {
    pub fn into_iter(self) -> MyIntoIter<T> {
        MyIntoIter(self)
    }
}

impl<T> Iterator for MyIntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()

    }
}


pub struct MyIter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<T> List<T> {
    pub fn iter<'a>(&'a self) -> MyIter<'a, T> {
        MyIter {next: self.head.as_deref() }
    }
}


impl<'a, T> Iterator for MyIter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| { 
            self.next = node.next.as_deref();
            &node.elem 
        })
    }
}