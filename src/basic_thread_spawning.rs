use std::thread;
use std::time::Duration;

pub fn basic_thread_spawning() {
    let t1 = thread::spawn(|| {
        let thread_id = thread::current().id();
        thread::sleep(Duration::from_secs(2));
        println!("large execution thread {:?}", thread_id);
    });
    let t2 = thread::spawn(nex_thread);

    println!("the main thread is {:?}", thread::current().id());

    t1.join().unwrap();
    t2.join().unwrap();
}

fn nex_thread() {
    let thread_id = thread::current().id();
    println!("spawn a thread {:?}", thread_id);
}

pub fn basic_thread_spawning2() {
    let mut handlers = vec![];

    for i in 0..10 {
        let handler = thread::spawn(move || {
            let thread_id = thread::current().id();
            thread::sleep(Duration::from_secs(2));
            println!("large execution thread {:?} {i}", thread_id);
        });

        handlers.push(handler);
    }

    for handler in handlers {
        handler.join().unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_thread_spawning() {
        basic_thread_spawning();
    }

		#[test]
    fn test_basic_thread_spawning2() {
      basic_thread_spawning2();
    }
}
