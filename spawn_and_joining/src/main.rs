
use std::{sync::atomic::{AtomicI32, Ordering::Relaxed}, thread};

static X: AtomicI32 = AtomicI32::new(0);

fn main() {
    println!("Hello, world!");

    X.store(1, Relaxed);

    let t = thread::spawn(f);

    X.store(2, Relaxed);

    t.join().unwrap();

    X.store(3, Relaxed);

    
}

fn f() {
    let x = X.load(Relaxed);
    assert!(x==1 || x==2);
}
