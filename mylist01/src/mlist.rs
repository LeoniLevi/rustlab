

pub type MyList = Option<MyNode>;
pub struct MyNode {
	pub elem: i32,
    pub next: Box<MyList>
}

pub fn mlist_empty() -> Box<MyList> {
	Box::new(None)
}

pub fn mlist_from_vec(nums: &Vec<i32>) -> Box<MyList> {
	let mut iter = nums.iter().rev();	
	mlist_from_iter(&mut iter)
}


pub fn mlist_from_iter(nums: &mut dyn Iterator<Item = &i32>) -> Box<MyList>{
	let mut lst = mlist_empty();
	
	let mut val : Option<&i32>= nums.next();
	while val.is_some() {
		lst = mlist_add_elem(lst, *val.unwrap());
		val = nums.next();
	}
	lst
}


pub fn mlist_add_elem(list: Box<MyList>, num: i32) -> Box<MyList> {	
	let node = MyNode {elem : num, next : list};
	Box::new(Some(node))
}

pub fn mlist_get_head(list: &MyList) -> i32 {
    match list {
        Some(node) => node.elem,
        None => {panic!("get_head: MList is EMPTY");}
    }
}

pub fn mlist_get_tail(list: &MyList) -> &MyList {
    match list {
        Some(node) => &node.next,
        None => {panic!("get_head: MList is EMPTY");}
    }
}

pub fn mlist_concat(lst1: &MyList, lst2: Box<MyList>) -> Box<MyList> {
	if mlist_is_empty(lst1) {
		return lst2;
	}
	
	let first = mlist_get_head(lst1);
	let m_tail = mlist_get_tail(lst1);
	
	
	let mrest = mlist_concat(m_tail, lst2);
	mlist_add_elem(mrest, first)	
}

pub fn mlist_reverse(lst: &MyList) -> Box<MyList> {
	if mlist_is_empty(lst) {
		return mlist_empty();
	}
	let n = mlist_get_head(lst);
	let nlst = mlist_one(n);
	
	let t = mlist_get_tail(&lst);	
	let rt = mlist_reverse(t);
	mlist_concat(&rt, nlst)
}


pub fn print_mlist(lst: &MyList, list_name: &str) {
	println!(" ~~~ mlist '{}':", list_name);
	print_vals(&lst);
    println!();
}

//---------------


fn mlist_is_empty(list: &MyList) -> bool {
	list.is_none()
}

fn mlist_one(num: i32) -> Box<MyList> {
	mlist_add_elem(mlist_empty(), num)
}

fn print_vals(list: &MyList) {
	if list.is_some() {
		let val = mlist_get_head(list);
		print!(" {} ", val);
		let tail = mlist_get_tail(list);
		print_vals(tail);
	}
}

