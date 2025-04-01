use std::{sync::atomic::{fence, AtomicI32, Ordering}, thread};

static A: AtomicI32 = AtomicI32::new(0);
static B: AtomicI32 = AtomicI32::new(0);

fn thread_1() {
    fence(Ordering::Release);
    A.store(1, Ordering::Relaxed);
    B.store(2, Ordering::Relaxed);
}

fn thread_2() {
    let a = A.load(Ordering::Relaxed);
    let b = B.load(Ordering::Relaxed);
    fence(Ordering::Acquire);
}

fn main() {
    thread::scope(|s|{
        s.spawn(||{
            thread_1();
        });

        s.spawn(||{
            thread_2();
        });
    });


}