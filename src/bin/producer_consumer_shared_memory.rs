use std::thread;
use std::sync::{Arc, Mutex};


fn main() {
    let nums : Vec<i32> = Vec::new();
    let data = Arc::new(Mutex::new(nums));
    let cpus = num_cpus::get();
    //exit is a signal
    let exit = -99999;
    let (mut sptx, sprx) = spmc::channel();
    let mut handles = Vec::new();

    //create all the threads
    for n in 0..cpus {
        let (data, rx) = (Arc::clone(&data), sprx.clone());

        handles.push(thread::spawn(move || {
            loop {
                let msg = rx.recv().unwrap();
                if msg == exit {
                    break
                }
                //do some calculation
                let mut value = 0;
                for i in 0..msg*2 {
                    value += i;
                }
                let mut data = data.lock().unwrap();
                data.push(value);
            }
        }));
    }

    //just send numbers
    for i in 0..1000 {
        sptx.send(i).unwrap();
    }

    //send the exit signal
    for i in 0..cpus {
        sptx.send(exit);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let data = data.lock().unwrap();
    println!("{:?}", data);
}