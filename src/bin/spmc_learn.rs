use std::thread;

fn main() {
    let cpus = num_cpus::get();
    let exit = -99999;
    let (mut tx, rx) = spmc::channel();

    let mut handles = Vec::new();
    for n in 0..cpus {
        let rx = rx.clone();
        handles.push(thread::spawn(move || {
            loop {
                let msg = rx.recv().unwrap();
                if msg == exit {
                    break
                }
                println!("worker {} recvd: {}", n, msg);
            }

        }));
    }

    for i in 0..1000 {
        tx.send(i).unwrap();
    }

    for i in 0..cpus {
        tx.send(exit);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}