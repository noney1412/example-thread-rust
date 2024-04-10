use std::cell::RefCell;
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

#[derive(Debug)]
struct MyData {
    value: String,
}

pub fn shared_with_rc() {
    let a = Rc::new(RefCell::new(MyData {
        value: "hello".to_string(),
    }));

    let b = a.clone();

    {
        let c = a.clone();

        println!(
            "a : {:?}, b: {:?}, c: {:?}",
            a.as_ptr(),
            b.as_ptr(),
            c.as_ptr()
        );

        let mut brm = c.borrow_mut();

        brm.value = "world".to_string();

        println!("ref count in scope: {:?}", Rc::strong_count(&a));
    }

    println!("ref count out scope: {:?}", Rc::strong_count(&a));
    println!("the latest value: {:?}", a);
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
