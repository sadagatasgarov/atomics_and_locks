use std::sync::atomic::Ordering::Relaxed;
use std::sync::atomic::{AtomicI32, AtomicU32};

fn main() {
    let at = AtomicU32::new(2);
    allocate_new_id();
    allocate_new_id();
    let a = allocate_new_id();
    println!("{a}");
    allocate_new_id();
    let a = allocate_new_id();
    println!("{a}");

    increment(&at);
    println!("{:?}", at);
}

// fn allocate_new_id() -> u32 {
//     static NEXT_ID: AtomicU32 = AtomicU32::new(0);
//     NEXT_ID.fetch_add(1, Relaxed)
// }

fn allocate_new_id() -> u32 {
    static NEXT_ID: AtomicU32 = AtomicU32::new(0);
    let id = NEXT_ID.fetch_add(1, Relaxed);
    if id >= 100 {
        NEXT_ID.fetch_sub(1, Relaxed);
        panic!("too many IDS!");
    }

    id
}

fn increment(a: &AtomicU32) {
    let mut current = a.load(Relaxed);
    loop {
        let new = current + 1;
        match a.compare_exchange(current, new, Relaxed, Relaxed) {
            Ok(_) => return,
            Err(v) => current = v,
        }
    }
}
