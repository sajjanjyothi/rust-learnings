use std::collections::HashMap;
use std::sync::Mutex;
use std::fs::File;
use std::io::Read;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;
#[allow(unused_imports)]
use std::thread::sleep;
#[allow(unused_imports)]
use std::time::Duration;
use component::test_package::{CheckError};
use component::checker::Sajjan;

mod component;

fn main() -> Result<(), CheckError> {
    let array :[i32;5] = [1,2,3,4,5];
    //vector
    let  mut my_vec:Vec<Sajjan> = Vec::new();
    my_vec.push(Sajjan{
        name: "test1".to_string(),
        age: 10,
    });

    my_vec.push(Sajjan{
        name: "test2".to_string(),
        age: 33,
    });

    //hash map
    let mut map_details = HashMap::new();
    //key value
    map_details.insert("test1".to_string(),"test2".to_string());
    let map_value = map_details.get("test1").unwrap();
    println!("map value --- {}",map_value);

    let (send,rcv): (Sender<i32>,Receiver<i32>) = channel();
    send.send(34).unwrap();
    send.send(35).unwrap();
    let mt = Mutex::new("sajjan test");
    let s1 = mt.lock().unwrap();
    println!("Hello, world!");
    eprintln!("sajjan here");
    println!("{}", component::test_package::say_hello());
    let val = component::test_package::check_value().or_else(|_| {Ok(true)})?;
    println!("{}",val);
    let s = String::from("my value");
    component::test_package::check_borrow(&s);
    println!("after borrow {}",s1);
    let sajjan_obj = component::checker::new("sajjan jyothi".to_string());
    checker(sajjan_obj);
    println!("---------------file contents---------------");
    println!("{}",read_file("sajjan.txt".to_string()));
    let t = thread::spawn(||{
        for val in rcv  {
           println!("{}", val);
            if val == 99{
                break;
            }
        }  
    });
    for i in 1.. 100{
       send.send(i).unwrap();
    }
    // thread::sleep(Duration::new(10,0));
    t.join().expect("cannot join");
    // println!("{}", rcv.recv().unwrap());
    //Err(HomeError::Good)
    for val in  array{
        println!("{}",val)
    }

    for val in my_vec{
        println!("{:?}",val)
    }
    Ok(())
}

fn checker(check : impl component::checker::Checker){
    let name = check.get_name();
    check.check_these(name.to_string());
}

fn read_file(filename: String) -> String{
    let mut s = String::new();
    let mut f = File::open(filename).expect("cannot found file");
    f.read_to_string(&mut s).expect("file read filed");
    s
}