use std::rc::*;
use std::cell::*;

pub struct DDList {
    pub first: DDLink,
    pub last: DDLink,
}

impl DDList {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self { first: DDLink::empty(), last: DDLink::empty()}
    }

    pub fn add_first(&mut self, value: i32) {
        if self.first.is_empty() {
            self.first = DDLink::new(value);
            self.last = self.first.clone();
        }
        else {
            self.first = self.first.add_prev(value);
        }
    }

    pub fn add_last(&mut self, value: i32) {
        if self.last.is_empty() {
            self.first = DDLink::new(value);
            self.last = self.first.clone();
        }
        else {
            self.last = self.last.add_next(value);
        }
    }

    pub fn iter(&self) -> DDListIter {
        DDListIter::new(self)
    }
}

pub struct DDListIter<'a> {
    lst: &'a DDList,
    fcur: DDLink, // forward-iteration 'cursor'
    bcur: DDLink, // backward iteration 'cursor'
}

impl<'a> DDListIter<'a> {
    pub fn new(my_lst: &'a DDList) -> Self {
        Self {lst: my_lst, fcur: DDLink::empty(), bcur: DDLink::empty()}
    }
}

impl<'a> Iterator for DDListIter<'a> {
    type Item = i32;
    fn next(&mut self) -> Option<Self::Item> {    
        let dnext;
        if self.fcur.is_empty() {
            dnext = self.lst.first.clone();
        }
        else {
            dnext = self.fcur.get_next();
        }
        if dnext.is_empty() || dnext.is_eqv(&self.bcur) {
            return None;
        }
        self.fcur = dnext;
        self.fcur.get_value()
    }
}

impl<'a> DoubleEndedIterator for DDListIter<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        let dnext;
        if self.bcur.is_empty() {
            dnext = self.lst.last.clone();
        }
        else {
            dnext = self.bcur.get_prev();
        }
        if dnext.is_empty() || dnext.is_eqv(&self.fcur) {
            return None;
        }
        self.bcur = dnext;
        self.bcur.get_value()
    }
}

pub struct DDListIntoIter {
    lst: DDList,
    cur: DDLink,
}
impl DDListIntoIter {
    pub fn new(my_lst: DDList) -> Self {
        Self {lst: my_lst, cur: DDLink::empty()}
    }
}

impl Iterator for DDListIntoIter {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cur.is_empty() {
            self.cur = self.lst.first.clone();
        }
        else {
            self.cur = self.cur.get_next();
        }
        self.cur.get_value()
    }
}

impl IntoIterator for DDList {
    type Item = i32;
    type IntoIter = DDListIntoIter;
    fn into_iter(self) -> Self::IntoIter {
        DDListIntoIter::new(self)
    }
}

pub struct DDLink {
    dl: DLink
}

impl DDLink {
    #[allow(dead_code)]
    pub fn new(value: i32) -> Self {
        Self {dl: create_dlink(value)}
    }

    #[allow(dead_code)]
    pub fn empty() -> Self {
        Self { dl: None}
    }
    pub fn is_empty(&self) -> bool {
        self.dl.is_none()
    }

    #[allow(dead_code)]
    pub fn is_eqv(&self, dlink: &DDLink) -> bool {
        if self.is_empty() && dlink.is_empty() {
            return true;
        }
        if self.is_empty() || dlink.is_empty() {
            return false;
        }
        //let src = self.as_ref().unwrap();
        return Rc::ptr_eq(self.dl.as_ref().unwrap(), dlink.dl.as_ref().unwrap());
    }

    #[allow(dead_code)]
    pub fn clone(&self) -> Self {
        Self{ dl: clone_dlink(&self.dl)}
    }
    #[allow(dead_code)]
    pub fn get_value(&self) -> Option<i32> {
        get_dlink_value(&self.dl)
    }

    #[allow(dead_code)]
    pub fn add_next(&self, next_val: i32) -> Self {
        Self{dl: add_dlink_next(&self.dl, next_val)}
    }

    #[allow(dead_code)]
    pub fn add_prev(&self, prev_val: i32) -> Self {
        Self{dl: add_dlink_prev(&self.dl, prev_val)}
    }

    pub fn has_next(&self) -> bool {
        has_dlink_next(&self.dl)
    }

    #[allow(dead_code)]
    pub fn get_next(&self) -> Self {
        Self{dl: get_next_dlink(&self.dl)}
    }

    //#[allow(dead_code)]
    pub fn get_prev(&self) -> Self {
        Self{dl: get_prev_dlink(&self.dl)}
    }
}


pub fn print_dlist_forward(dd: &DDLink) {
    let mut dlink = dd.clone();
    while !dlink.is_empty() {
        print!(" {} ", dlink.get_value().unwrap());
        dlink = dlink.get_next();
    }
    println!();
}

pub fn print_dlist_backward(dd: &DDLink) {
    let mut dlink = dd.clone();
    while !dlink.is_empty() {
        print!(" {} ", dlink.get_value().unwrap());
        dlink = dlink.get_prev();
    }
    println!();
}

pub fn find_dlist_last(dd: &DDLink) -> DDLink {
    let mut dlink = dd.clone();
    while dlink.has_next() {
        dlink = dlink.get_next();
    }
    return dlink;
}


//=============================

type DLink = Option<Rc<RefCell<DNode>>>;

// #[allow(dead_code)]
 struct DNode {
    val: i32,
    next: Option<Rc<RefCell<DNode>>>,
    prev: Option<Weak<RefCell<DNode>>>
}

impl DNode {
    #[allow(dead_code)]
    pub fn new(value: i32) -> Self {
        Self {val: value, next: None, prev: None}
    }
}

fn create_dlink(value: i32) -> DLink {
    Some(Rc::new(RefCell::new(DNode::new(value))))
}

fn get_dlink_value(dlink: &DLink) -> Option<i32> {
    dlink.as_ref().map(|rcn|rcn.borrow().val)
}

fn get_dlink_node(dlink: &DLink) -> &Rc<RefCell<DNode>> {
    dlink.as_ref().unwrap()
}
fn build_wdlink_node(wdlink: &Option<Weak<RefCell<DNode>>>) -> Rc<RefCell<DNode>> {
    wdlink.as_ref().unwrap().upgrade().unwrap()
}
fn build_dlink_weak(dlink: &DLink) -> Weak<RefCell<DNode>> {
    let node = dlink.as_ref().unwrap();
    Rc::downgrade(node)
}

fn add_dlink_next(dlink: &DLink, next_val: i32) -> DLink {
    let new_dlink = create_dlink(next_val);
    
    let node = get_dlink_node(dlink);
    let mut b_node = node.borrow_mut();
    {
        let mut b_new_node = get_dlink_node(&new_dlink).borrow_mut();
        if b_node.next.is_some() {
            let mut b_old_next = get_dlink_node(&b_node.next).borrow_mut();
            b_old_next.prev = Some(build_dlink_weak(&new_dlink));
            
            b_new_node.next = clone_dlink(&b_node.next);   
        }       
        b_new_node.prev = Some(build_dlink_weak(dlink));
    }
    b_node.next = clone_dlink(&new_dlink);//Some(new_node);
    return new_dlink;
}

//#[allow(dead_code)]
fn add_dlink_prev(dlink: &DLink, prev_val: i32) -> DLink{
    let new_dlink = create_dlink(prev_val);
    let mut b_node = get_dlink_node(dlink).borrow_mut();
    {
        let mut b_new_node = get_dlink_node(&new_dlink).borrow_mut();
        if b_node.prev.is_some() {
            let prev_rc = build_wdlink_node(&b_node.prev);
            {
                let mut b_old_prev = prev_rc.borrow_mut();
                b_old_prev.next = clone_dlink(&new_dlink);
            }
            b_new_node.prev = Some(Rc::downgrade(&prev_rc));
        }
        b_new_node.next = clone_dlink(dlink);
    }
    b_node.prev = Some(build_dlink_weak(&new_dlink));
    return new_dlink;
}

fn clone_dlink(dlink: &DLink) -> DLink {
    dlink.as_ref().map(|rcn| rcn.clone())
}

fn has_dlink_next(dlink: &DLink) -> bool {
    if dlink.is_some() {
        let b_node = get_dlink_node(dlink).borrow();
        if b_node.next.is_some() {
            return true;
        }
    }
    return false;
}

fn get_next_dlink(dlink: &DLink) -> DLink {
    if dlink.is_some() {
        let b_node = get_dlink_node(dlink).borrow();
        if b_node.next.is_some() {
            return clone_dlink(&b_node.next);
        }
    }
    return None;
}

fn get_prev_dlink(dlink: &DLink) -> DLink {
    if dlink.is_some() {
        let b_node = get_dlink_node(dlink).borrow();
        if b_node.prev.is_some() {
            return Some(build_wdlink_node(&b_node.prev));
        }
    }
    return None;
}

fn print_dlist(dlink: &DLink) {
    if dlink.is_some() {
        let b_node = get_dlink_node(dlink).borrow();
        print!(" {} ", b_node.val);
        print_dlist(&b_node.next);
    }
}


#[allow(dead_code)]
fn play_with_dlist0() {
   let dlink0 = create_dlink(5);
   add_dlink_next(&dlink0, 8);
   add_dlink_next(&dlink0, 7);
   add_dlink_next(&dlink0, 3);
   add_dlink_next(&dlink0, 1);
   add_dlink_next(&dlink0, -1);
   
   println!(" ~~ print_dlist ...");
   print_dlist(&dlink0);
   println!();

   let mut dlink99 = create_dlink(0);
   println!(" ~~ Forward Iteration ...");
   let mut dlink = clone_dlink(&dlink0);
   while dlink.is_some() {
       dlink99 = clone_dlink(&dlink);

       let val = get_dlink_value(&dlink).unwrap();
       print!(" {} ", val);
       dlink = get_next_dlink(&dlink);
   }
   println!();

   println!(" ~~ Backward Iteration ...");
   dlink = clone_dlink(&dlink99);
   while dlink.is_some() {
       let val = get_dlink_value(&dlink).unwrap();
       print!(" {} ", val);
       dlink = get_prev_dlink(&dlink);
   }
   println!();

   println!(" ~~ play_with_dlist - completed");
}
