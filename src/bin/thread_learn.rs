use std::thread;
use std::time::Duration;
use std::collections::LinkedList;
extern crate num_cpus;

fn main() {

    let num = num_cpus::get();

    let mut handles = LinkedList::new();

    for i in 1..num {

        // Using a closure so that we can create a parameter
        let th = |i| {
            for j in 1..10 {
                println!("hi number {} from the spawned thread! {}", j,  i);
                thread::sleep(Duration::from_millis(1));
            }
        };

        // Using move so that join can take ownership of the closure with i parameter
        let handle = thread::spawn(move || th(i));
        handles.push_back(handle);
    }


    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
}