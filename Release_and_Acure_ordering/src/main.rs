
use std::{sync::atomic::{AtomicBool, AtomicU64, Ordering::{Acquire, Relaxed, Release}}, thread, time::Duration};


// static DATA: AtomicU64 = AtomicU64::new(0);
// static READY: AtomicBool = AtomicBool::new(false);


// fn main() {
//     thread::spawn(|| {
//         DATA.store(123, Relaxed);
//         READY.store(true, Release);
//     });

//     while !READY.load(Acquire)  {
//         thread::sleep(Duration::from_millis(100));
//         println!("waiting...")  
//     }

//     println!("{}", DATA.load(Relaxed));

// }


static mut DATA: u64 = 0;
static READY: AtomicBool = AtomicBool::new(false);


fn main() {
    thread::spawn(|| {
        unsafe  {
            DATA = 123
        };

        READY.store(true, Release);
    });

    while !READY.load(Acquire) {
        thread::sleep(Duration::from_millis(100));
        println!("waiting...");
    }

    println!("{}", unsafe {
        DATA
    });
}