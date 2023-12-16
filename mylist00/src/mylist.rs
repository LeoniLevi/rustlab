
type MyList = Option<SNode>;
pub struct SNode {
	val: i32,
	next: Box<MyList>,
}

pub fn empty_list() -> Box<MyList> { Box::new(None) }

pub fn add_item(list: Box<MyList>, num: i32) -> Box<MyList> {
	Box::new( Some( SNode{ val: num, next: list}))
}

pub fn vec_to_list(vec: Vec<i32>) -> Box<MyList> {
	let mut lst = empty_list();
	for n in vec.iter().rev() {
		lst = add_item(lst, *n);
	}
	lst
}

pub fn get_val_by_idx(list: &MyList, idx: i32) -> Option<i32> {
	let mut lst = list;
	let mut count = 0;
	while let Some(node) = lst {
		if count == idx {
			return Some(node.val);
		}
		count += 1;
		lst = &node.next;
	}
	None
}


pub fn list_len(list: &MyList) -> i32 {
	if list.is_some() {1 + list_len(get_tail0(list))} else {0}
}

pub fn print_list(list: &MyList) {
	println!(" ~~ print_list:");
	let mut lst = list;
	while let Some(node) = lst {
		print!("{} ", node.val);
		lst = get_tail0(lst);
	}
	println!();
}

pub fn get_head(list: &MyList) -> i32 {
	match list {
		Some(node) => node.val,
		_ => panic!("!! get_head0 !!")
	}
}
pub fn get_tail0(list: &MyList) -> &MyList {
	match list {
		Some(node) => &node.next,
		_ => panic!("!! get_tail0 !!")

	}
}

pub fn print_list0(list: &MyList) {
	println!(" ~~ print_list0:");
	let mut lst = list;
	while lst.is_some() {
		let n = get_head(lst);
		print!("{}", n);
		lst = get_tail0(lst);
		if lst.is_none() {
			break;
		}
		print!(", ")
	}
	println!()
}

pub fn print_list1(list: &MyList) {
	println!(" ~~ print_list1:");
	list.as_ref().map(|n| {
		prn_lst(n);
	});
}

fn prn_lst(node: &SNode) {
	print!("{}", node.val);
	let next_node = node.next.as_ref();
	next_node.as_ref().map(|n| {
		print!(",");
		prn_lst(n);
	});
}

