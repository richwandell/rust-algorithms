use std::thread;
use std::sync::{mpsc};


fn main() {
    let cpus = num_cpus::get();
    //exit is a signal
    let exit = -99999;
    let (mut sptx, sprx) = spmc::channel();
    let (mut mptx, mprx) = mpsc::channel();
    let mut handles = Vec::new();

    //create all the threads
    for n in 0..cpus {
        let rx = sprx.clone();
        let tx = mptx.clone();
        handles.push(thread::spawn(move || {
            loop {
                let msg = rx.recv().unwrap();
                if msg == exit {
                    tx.send(exit);
                    break
                }
                //do some calculation
                let return_value = msg * msg * msg;
                tx.send(return_value);
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

    //recieve all messages
    let mut finished = 0;
    loop {
        let msg = mprx.recv().unwrap();
        if msg == exit {
            finished += 1;
            if finished == cpus {
                break
            }
        } else {
            println!("{}", msg);
        }

    }

    for handle in handles {
        handle.join().unwrap();
    }
}