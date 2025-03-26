use std::sync::atomic::Ordering::Relaxed;
use std::sync::atomic::{AtomicI32, AtomicU32};

fn main() {
    allocate_new_id();
    allocate_new_id();
    let a = allocate_new_id();
    println!("{a}");
    allocate_new_id();
    let a = allocate_new_id();
    println!("{a}")
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
