//pub type MyList1 = Option<Box<Node1>>;
pub struct MyList1 {
    head: Option<Box<Node1>>,
}

pub struct Node1 {
    val : i32,
    next: Option<Box<Node1>>,
}

pub fn make_empty_list1() -> MyList1 {
    MyList1 {head: None}
}

pub fn list1_add(lst: &mut MyList1, val: i32) {
    lst.head = Some (Box::new(Node1{val: val, next: lst.head.take()}))
}

impl MyList1 {
    pub fn new() -> Self {
        make_empty_list1()
    }
    pub fn add(&mut self, val: i32) {
        list1_add(self, val);
        //self.head = Some (Box::new(Node1{val: val, next: self.head.take()}))
    }
    pub fn iter(&self) -> MyList1Iter {
        MyList1Iter::new(&self)
    }
}


pub struct MyList1Iter<'a> {
    cur: &'a Option<Box<Node1>>,
}

impl<'a> MyList1Iter<'a> {
    pub fn new(lst: &'a MyList1) -> Self {
        Self {cur: &lst.head}
    }
}

impl<'a> Iterator for MyList1Iter<'a> {
    type Item = i32;
    fn next(&mut self) -> Option<Self::Item> {
        if self.cur.is_none() {
            None
        } else {
            let val: i32 = self.cur.as_ref().unwrap().val;
            self.cur = &self.cur.as_ref().unwrap().next;
            Some(val)
        }
    }
}
