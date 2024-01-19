extern crate rand;

mod mydlist;
use mydlist::*;

mod myrect;
use myrect::*;

mod mylist;
use mylist::*;

mod mylist1;
use mylist1::*;

use std::collections::LinkedList;

fn play_with_std_list()
{
    let mut list: LinkedList<u32> = LinkedList::new();

    list.push_back(9);
    list.push_back(1);
    list.push_back(2);
    list.push_back(4);

    let mut itr = list.iter();
    let mut val = itr.next().unwrap();
    println!(" ~~ itr.next={}", val);

    val = itr.next().unwrap();
    println!(" ~~ itr.next={}", val);

    let mut ritr = list.iter().rev();
    val = ritr.next().unwrap();
    println!(" ~~ ritr.next={}", val);

    val = ritr.next().unwrap();
    println!(" ~~ ritr.next={}", val);
/*
    let cursor = list.cursor_front();
    cursor.move_next();
    let val = cursor.current().unwrap();
    println!(" ~~ cursor.current={}", val);
*/



    println!(" ~~ play_with_std_list - completed");
}

fn play_with_array() {
    let anums = [2, 4, 6, 8, 10, 12];
    print!("Array iter: {{");
    for num in anums.iter() {
        print!(" {} ", num);
    }
    println!("}}");

    print!("Array iter.rev: {{");
    for num in anums.iter().rev() {
        print!(" {} ", num);
    }
    println!("}}");

    let vnums = vec!(3, 6, 9, 12, 15);
    print!("Vector iter: {{");
    for num in vnums.iter() {
        print!(" {} ", num);
    }
    println!("}}");
/*
    print!("Vector: {{");
    for num in vnums {
        print!(" {} ", num);
    }
    println!("}}");
*/
    print!("Vector iter.rev: {{");
    for num in vnums.iter().rev() {
        print!(" {} ", num);
    }
    println!("}}");

    println!(" ~~ play_with_array - completed");
}

fn play_with_list() {
	let nums = vec!(22, 33, 44, 55, 66);
    let lst = vec_to_list(nums);
    
    let len = list_len(&lst);
    println!(" ~~ list: len={}, head={}", 
             len, get_head(&lst).unwrap());
    let v = get_val_by_idx(&lst, 1).unwrap();
    println!(" ~~ list[1] = {}", v);
    
    println!(" ~~ list:");
	print_list0(&lst);
    print_list(&lst);
    
    println!(" ~~ tail(list) [aca CDR]:");
    let tlist = get_tail(&lst).unwrap();
    print_list(&tlist);

    println!(" ~~ reversed list:");
	print_rev_list0(&lst);
}

fn play_with_ddlist() {
    let mut lst = DDList::new();
    lst.add_first(2);
    lst.add_first(3);
    lst.add_first(4);

    lst.add_last(7);
    lst.add_last(8);
    lst.add_last(9);

    println!(" ~~ DDList: print_dlist_forward ...");
    print_dlist_forward(&lst.first);

    println!(" ~~ DDList: print_dlist_backward ...");
    print_dlist_backward(&lst.last);

    println!(" ~~ DDList: DDListIter Iteration ...");
    for val in lst.iter() {
        print!("{}; ", val);
    }
    println!();

    println!(" ~~ DDList: DDListIter REV Iteration ...");
    for val in lst.iter().rev() {
        print!(" {} ", val);
    }
    println!();


    println!(" ~~ ITER FW+BW testing...");    
    let mut fit = lst.iter();
    /*
    for i in 1..5 {
        let r = fit.next_back();
        println!(" BW {}.  {:?}", i, r);
    }
*/
    for i in 1..5 {
        let r = fit.next();
        println!(" FW {}.  {:?}", i, r);
    }   

    for i in 1..5 {
        let r = fit.next_back();
        println!(" BW {}.  {:?}", i, r);
    }

    println!(" ~~ DDList: FOR<into_iter> Iteration ...");
    for val in lst {
        print!("{}; ", val);
    }
    println!();

    println!(" ~~ play_with_ddlist - completed");
}

fn play_with_dlist() {
    let dlink0 = DDLink::new(5);
    
    dlink0.add_next(8);
    dlink0.add_next(7);

    let dn = dlink0.add_next(3);
    assert_eq!(dn.get_value().unwrap(), 3);
    let np = dn.get_prev();
    println!(" DBG np.value = {}", np.get_value().unwrap());
    assert!(np.get_value().unwrap() == 5);

    dlink0.add_next(1);
    dlink0.add_next(-1);

    let dd = dlink0.add_prev(29);
    
    let dlink_last = find_dlist_last(&dd);

    println!(" ~~ Forward TEST Iteration ...");
    print_dlist_forward(&dd);    

    println!(" ~~ Forward Iteration ...");
    print_dlist_forward(&dlink0);

    println!(" ~~ Backward Iteration ...");
    print_dlist_backward(&dlink_last);
    
    println!(" ~~ play_with_dlist - completed");
}

fn play_with_list1() {
    println!(" !! play_with_list1 started..");

    //let mut lst = make_empty_list1();
    let mut lst = MyList1::new();
    lst.add(16);
    lst.add(17);
    lst.add(18);
    lst.add(19);
    
    for v in lst.iter() {
        println!("v={}", v);
    }

    for v in lst.iter_mut() {
        let n = *v;
        *v = n + 10;
        println!("{} -> {}", n, *v);
    }

    let mut itr = lst.iter();
    loop {
        let ov = itr.next();
        if ov.is_none() {
            break;
        }
        println!("vv = {}", ov.unwrap());
    }

    let mut itr1 = lst.iter();
    let mut ovv = itr1.next();
    while ovv.is_some() {
        println!("vvv = {}", ovv.unwrap());
        ovv = itr1.next()
    }
    
    println!(" !! play_with_list1 - completed");
}

fn main() {
    println!(" === Program: Started...");
    let i: i8 = rand::random();
    println!("The random i8 is {}", i);

    loop {
        println!("\n ---------------------------------------");
        println!(" ~~ Here are your options: "); 
        println!(" 0. EXIT");
        println!(" 1. play List (singly linked list)");
        println!(" 2. play List1 (singly linked list with methods)");
        println!(" 3. play DList (doubly linked list)");
        println!(" 4. play DDList (doubly linked list with methods)");
        println!(" 5. play StdList (standard library list)");
        println!(" 6. play Array");
        println!(" 7. draw Rect");
        println!(" ~~ Make your choice(0 - 7): ");

        let mut sbuf = String::new();
        std::io::stdin().read_line(&mut sbuf).unwrap();
        let schoice = sbuf.trim_end();
        let choice : i32 = schoice.parse().unwrap();
        println!("Chosen: {}", choice); 

        println!(" ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~");
        match choice {
            0 => println!(" ~~ EXIT is chosen!"),
            1 => play_with_list(),
            2 => play_with_list1(),
            3 => play_with_dlist(),
            4 => play_with_ddlist(),
            5 => play_with_std_list(),
            6 => play_with_array(),
            7 => draw_rect(10),
            _ => println!(" Choice is unknown!")
        }
        if choice == 0 { break; }
    }

/*
	play_with_list();
	draw_rect(8);
	//draw_rect(12);

    play_with_ddlist();

    play_with_dlist();

    play_with_array();

    play_with_std_list();

    play_with_list1();
*/
    println!("\n === Program: Completed. Bye!")
}
