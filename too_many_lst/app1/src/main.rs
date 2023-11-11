mod first;
mod second;
use first::List as ListOne;

use second::List as ListTwo;

fn main() {
    println!("App1 started..");
    test_one();
    test_two();
    println!("App1 completed.");
}

fn test_two() {
    println!(" ==== test_two started..");

    let mut list = ListTwo::new();
    assert_eq!(list.peek(), None);
    assert_eq!(list.peek_mut(), None);
    list.push(1); list.push(2); list.push(3);

    assert_eq!(list.peek(), Some(&3));
    assert_eq!(list.peek_mut(), Some(&mut 3));

    //let last = list.peek_mut();
    println!(" Last is: {}", list.peek().unwrap());

    list.peek_mut().map(|value| {
        *value = 42
    });
    println!(" Now the iModified Last is: {}", list.peek().unwrap());

    print!(" ~~ IntoIter iteration:");
    let mut iter0 = list.into_iter();
    loop {
        let val = iter0.next();
        if val.is_none() {
            break;
        }
        print!(" {} ", val.unwrap());
    }
    println!();
    

    list = ListTwo::new();
    list.push(1); list.push(2); list.push(3);

    print!(" ~~ Iter iteration:");
    let mut iter1 = list.iter();
    loop {
        let val = iter1.next();
        if val.is_none() {
            break;
        }
        print!(" {} ", val.unwrap());
    }
    println!();

    print!(" ~~ sequential list pop: ");
    while let Some(val) = list.pop() {
        print!("{val} ");
    }
    println!();


    println!(" ==== test_two completed");
}

fn test_one() {
    println!(" ==== test_one started...");
    {

        {
            let mut list = ListOne::new();
            assert_eq!(list.pop(), None);    
    
            list.push(11);
            list.push(12);
            list.push(13);   

            while let Some(val) = list.pop() {
                print!("{val} ");
            } 
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
    println!(" ==== test_one - completed");
}
