use std::thread;
use std::time::Duration;

fn main() {
    let t1 = thread::spawn(|| {
        let thread_id = thread::current().id();
        thread::sleep(Duration::from_secs(2));
        println!("large execution thread {:?}", thread_id);
    });
    let t2 = thread::spawn(|| hello());

    println!("the main thread is {:?}", thread::current().id());

    t1.join().unwrap();
    t2.join().unwrap();
}

fn hello() {
    let thread_id = thread::current().id();
    println!("spawn a thread {:?}", thread_id);
}