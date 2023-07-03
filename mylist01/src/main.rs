mod mlist;
use mlist::*;


fn mdo_list_work() {
	println!(" ==~~~== do_mlist_work...");
    let mut lst: Box<MyList> = mlist_empty();
    lst = mlist_add_elem(lst, 22);
    lst = mlist_add_elem(lst, 33);
    lst = mlist_add_elem(lst, 44);
    
    let tail = mlist_get_tail(&lst as &MyList);
    println!("Head is: {}", mlist_get_head(&lst));
    println!("Head-Head is: {}", mlist_get_head(&tail));
    
    
    print_mlist(&lst, "lst"); 
    let lst_r = mlist_reverse(&lst);
    print_mlist(&lst_r, "lst_r");    
    
    let v = vec![90, 91, 92, 93, 94, 95];
    let v_lst = mlist_from_vec(&v);    
    print_mlist(&v_lst, "v_lst");
    
    let sum_lst = mlist_concat(&v_lst, lst);
    print_mlist(&sum_lst, "sum_lst");
    
    print_mlist(&mlist_reverse(&sum_lst), "sum_lst<r>");
    
    
}

//////////////////

fn main() {
    println!(" ~~ mylist01-App started..");
    
    mdo_list_work();

    println!(" ~~ mylist01-App completed. Bye!");
}


