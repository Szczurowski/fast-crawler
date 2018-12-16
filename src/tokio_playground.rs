extern crate futures;
extern crate tokio_executor;

#[allow(dead_code)]
fn dox() {
    use futures::future::lazy;
    use tokio_executor::{ spawn, DefaultExecutor, Executor };

    let lazy_future = lazy(|| {
        println!("running on the default executor");
        Ok(())
    });
    let result = DefaultExecutor::current().spawn(Box::new(lazy_future));
    match result {
        Ok(_) => println!(""),
        Err(error) => panic!("Error on spawn {}", error)
    };
}

#[allow(dead_code)]
fn engage_threads() {
    use std::thread;
    use std::time::Duration;

    thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test01() {
        super::dox();
    }

    #[test]
    fn engage_threads_completes() {
        super::engage_threads();
    }
}
