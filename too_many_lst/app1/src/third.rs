use std::rc::Rc;

pub struct List<T> {
    head: Option<Rc<Node<T>>>,
}

//type Link<T> = Option<Rc<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Option<Rc<Node<T>>>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn prepend(&self, elem: T) -> List<T> {
        List {
            head: Some(Rc::new(Node {
                elem: elem, 
                next: self.head.clone(),
            }))
        }
    }

    pub fn tail(&self) -> List<T> {
        List { head: self.head.as_ref().and_then(|node| node.next.clone()) }
    } 
    
    pub fn head(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.elem)
    }

    pub fn iter(&self) -> MyIter<'_, T> {
        MyIter { next: self.head.as_deref()}
    }
}

struct MyIter<'a, T> {
    next: Option<&'a Node<T>>,
}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn basics() {
        let lst = List::new();
        assert_eq!(lst.head(), None);
        
        let lst = lst.prepend(4).prepend(5).prepend(6);
        assert_eq!(lst.head(), Some(&6));
        
        //let lst = lst.tail();
        assert_eq!(lst.tail().head(), Some(&5));
        assert_eq!(lst.tail().tail().tail().head(), None);
    }
}