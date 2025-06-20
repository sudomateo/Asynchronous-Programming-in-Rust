use std::thread::{self, sleep};

fn main() {
    println!("So, we start the program here!");
    let t1 = thread::spawn(move || {
        sleep(std::time::Duration::from_millis(200));
        println!("The long running tasks finish last!");
    });

    let t2 = thread::spawn(move || {
        sleep(std::time::Duration::from_millis(100));
        println!("We can chain callbacks...");
        let t3 = thread::spawn(move || {
            sleep(std::time::Duration::from_millis(50));
            println!("...like this!");
        });
        t3.join().unwrap();
    });
    println!("The tasks run concurrently!");

    let t4 = thread::spawn(move || {
        println!("Thread 4 started.");
        sleep(std::time::Duration::from_millis(500));
        println!("Thread 4 finished!");
    });

    t1.join().unwrap();
    t2.join().unwrap();
    t4.join().unwrap();
}
