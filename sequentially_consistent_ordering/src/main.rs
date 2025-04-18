use std::sync::atomic::AtomicI32;
use std::sync::atomic::Ordering::Relaxed;
use std::thread;

static X: AtomicI32 = AtomicI32::new(0);
fn main() {
    X.store(1, Relaxed);
    let t = thread::spawn(f);
    X.store(2, Relaxed);
    X.store(3, Relaxed);
    
    t.join().unwrap();
}

fn f() {
    let x = X.load(Relaxed);
    assert!(x == 3);
}
