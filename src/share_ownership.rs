use std::rc::Rc;
use std::thread;

pub fn static_share() {
    static A: i32 = 50;

    println!("A: {}", A);

    thread::spawn(|| println!("thread_id: {:?}, A: {}", thread::current().id(), A))
        .join()
        .unwrap();
    thread::spawn(|| println!("thread_id: {:?}, A: {}", thread::current().id(), A))
        .join()
        .unwrap();
}

pub fn shared_with_rc() {
    let a = Rc::new("my value".to_string());

    let b = a.clone();

    {
        let c = a.clone();
        println!("a : {:?}, b: {:?}, c: {:?}", a.as_ptr(), b.as_ptr(), c.as_ptr());
        println!("ref count in scope: {:?}", Rc::strong_count(&a));
    }

    println!("ref count out scope: {:?}", Rc::strong_count(&a));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_static_share() {
        static_share();
    }

    #[test]
    fn test_shared_with_rc() {
        shared_with_rc();
    }
}
