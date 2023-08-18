use std::collections::{HashMap, LinkedList};
use std::sync::Mutex;
use std::fs::File;
use std::io::Read;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::{env, thread};
#[allow(unused_imports)]
use std::thread::sleep;
#[allow(unused_imports)]
use std::time::Duration;
use component::test_package::{CheckError,say_hello,check_borrow, check_value};
use component::checker::{Checker,Sajjan,new};

mod component;

fn main() -> Result<(), CheckError> {
    //command line args
    let args: Vec<String> = env::args().collect();

    println!("command line args --- {:?}",args);

    //environment variable
    env::set_var("TEST_SAJJAN","HELLO");
    println!("env variable {}", env::var("TEST_SAJJAN").unwrap_or("default".to_string()));
    println!("env variable {}", env::var("TEST_SAJJAN_NOTSET").unwrap_or("default".to_string()));
    //infinite loop
    // loop{
    //
    // }
    //type conversion
    let a:i32 = 10;
    let b = a as u8;

    //array declaration
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
    let mut my_list = LinkedList::new();

    my_list.push_back(3);
    my_list.push_back(4);

    for val in my_list{
        println!("list -- {}",val);
    }

    //key value
    map_details.insert("test1".to_string(),"test2".to_string());
    let map_value = map_details.get("test1").unwrap();
    println!("map value --- {}",map_value);

    let map_value = map_details.get("test2");
    println!("contains --- {}",map_details.contains_key("test1-ooo"));
    //match check
    let val = match map_value {
        None => {String::from("ah no string")}
        Some(value) => {value.to_string()}
    };
    println!("{:?}",val);

    //unwrap or default value
    let val = map_value.unwrap_or(&"no value".to_string()).to_string();
    println!("{:?}",val);

    let (send,rcv): (Sender<i32>,Receiver<i32>) = channel();
    send.send(31).unwrap();
    send.send(35).unwrap();
    let struct_mt = Mutex::new(Sajjan{
        name: "test6".to_string(),
        age: 43,
    });
    let smt = struct_mt.lock().unwrap();
    println!("{}",smt.name);
    let mt = Mutex::new("sajjan test");
    let s1 = mt.lock().unwrap();
    println!("Hello, world!");
    eprintln!("sajjan here");

    println!("{}", say_hello());
    let val = check_value().or_else(|_| {Ok(true)})?;

    println!("{}",val);
    let s = String::from("my value");
    check_borrow(&s);
    println!("after borrow {}",s1);
    let sajjan_obj = new("sajjan jyothi".to_string());
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
    //move syntax , will get compilation error
   // thread::spawn(move ||{ println!("{}",map_details.get("test1").unwrap())});
    for val in  map_details {
        println!("{} -- {}",val.0,val.1);
    }
    sajjan_macro!("I am here hello world");

    //Box allocation - memory allocation
    let mut sajjan_ptr = Box::new(Sajjan{ name: "sajjan".to_string(), age: 0 });
    sajjan_ptr.age = 43;
    println!("{:?}",sajjan_ptr);
    let val = 67;
    let ref_check_value = life_time_check(&val);
    println!("{}",ref_check_value);
    Ok(())
}

fn checker(check : impl Checker){
    let name = check.get_name();
    check.check_these(name.to_string());
}

fn read_file(filename: String) -> String{
    let mut s = String::new();
    let mut f = File::open(filename).expect("cannot found file");
    f.read_to_string(&mut s).expect("file read filed");
    s
}

fn life_time_check<'a>(s :& 'a i32 ) -> & i32{
     s
}