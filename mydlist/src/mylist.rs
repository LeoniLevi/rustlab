
type MyList = Option<SNode>;
pub struct SNode {
	val: i32,
	next: Box<MyList>,
}

pub fn empty_list() -> Box<MyList> { Box::new(None) }
	

pub fn add_item(list: Box<MyList>, num: i32) -> Box<MyList> {
	let node = SNode {val: num, next: list};
	Box::new(Some(node))
}

pub fn vec_to_list(vec: Vec<i32>) -> Box<MyList> {
	let mut lst = empty_list();
	for n in vec.iter().rev() {
		lst = add_item(lst, *n);
	}
	lst
}

fn _get_head0(list: &Box<MyList>) -> i32 {
	match &**list {
		None => panic!("!!!"),
		Some(node) => node.val
	}
}

fn _get_tail0(list: &Box<MyList>) -> &Box<MyList> {
	match &**list {
		None => panic!("!!!"),
		Some(node) => &node.next
	}
}

pub fn get_val_by_idx(list: &Box<MyList>, idx: i32) -> Option<i32> {
	let mut lst = list;
	let mut count = 0;
	while let Some(node) = (**lst).as_ref() {
		if count == idx {
			return Some(node.val);
		}
		lst = &node.next;
		count += 1;
	}
	None
}

#[allow(dead_code)]
pub fn get_head_0(list: &Box<MyList>) -> Option<i32> {
	(**list).as_ref().map(|n| n.val)
}
// Use implicit coercion &Box<MyList> -->> &MyList...
pub fn get_head(list: &MyList) -> Option<i32> {
	list.as_ref().map(|n| n.val)
}

pub fn get_tail(list: &MyList) -> Option<&Box<MyList>> {
	list.as_ref().map(|n| &n.next)
}

pub fn list_len(list: &Box<MyList>) -> i32 {
	if list.is_none() {
		return 0;
	}
	return 1 + list_len(get_tail(list).unwrap());	
}

pub fn print_list(list: &Box<MyList>) {
	let mut lst = list;
	while let Some(node) = &**lst {
		print!("{} ", node.val);
		lst = get_tail(lst).unwrap();
	}
	println!();
}

pub fn get_head0(list: &MyList) -> i32 {
	match list {
		Some(node) => node.val,
		_ => panic!("!! get_head0 !!")
	}
	//list.unwrap().val 
}
pub fn get_tail0(list: &MyList) -> &MyList {
	match list {
		Some(node) => &node.next,
		_ => panic!("!! get_tail0 !!")

	}
	//&list.unwrap().next
}

pub fn print_list0(list: &MyList) {
	let mut lst = list;
	while lst.is_some() {
		let n = get_head0(lst);
		print!("{}", n);
		lst = get_tail0(lst);
		if lst.is_none() {
			break;
		}
		print!(", ")
	}
	println!()
}

pub fn print_rev_list0(list: &MyList) {
	if list.is_none() {
		return;
	}
	let n = get_head0(list);
	let t = get_tail0(list);
	print_rev_list0(t);
	if t.is_some() {
		print!(",");
	}
	print!("{}", n);

}
