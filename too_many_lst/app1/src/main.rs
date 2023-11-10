mod first;
use first::List as ListOne;


fn main() {
    println!("App1 started..");
    {

        {
            let mut list = ListOne::new();
            assert_eq!(list.pop(), None);    
    
            list.push(11);
            list.push(12);
            list.push(13);    
        }
        let mut list = ListOne::new();
        assert_eq!(list.pop(), None);    

        list.push(21);
        list.push(22);
        list.push(23);

        list.push(31);
        list.push(32);
        list.push(33);

        print!(" List elems: ");

        while let Some(val) = list.pop() {
            print!("{val} ");
        }

        // loop {
        //     let val = list.pop();
        //     if val.is_none() {
        //         break;
        //     }
        //     print!("{} ", val.unwrap());
        // }
        println!();
    }

    println!("App1 completed.");

}
