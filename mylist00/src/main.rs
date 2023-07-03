extern crate rand;

mod myrect;
//use myrect::draw_rect;
use myrect::*;

mod mylist;
use mylist::*;
/*
fn draw_rect(sz: u32) {
	println!();
	for i in 0 .. sz {
		for j in 0 .. sz {
			let mut c = '+';
			if (i + j) % 2 == 1 {
				c = '-';
			}
			print!(" {}", c);
		}
		println!();
	}
}
*/

fn play_with_list() {
	let nums = vec!(22, 33, 44, 55, 66);
    let lst = vec_to_list(nums);
    
    let len = list_len(&lst);
    println!(" ~~ list: len={}, head={}", len, get_head(&lst).unwrap());
    let v = get_val_by_idx(&lst, 1).unwrap();
    println!(" ~~ list[1] = {}", v);
    
    print_list(&lst);
	print_list0(&lst);
	print_list1(&lst);
}

fn main() {
    println!("Hello, world!");
    let i: i32 = rand::random();
    println!("The random i32 is {}", i);
    
	play_with_list();
	draw_rect(8);
	draw_rect(12);
}
