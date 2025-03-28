use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::{Relaxed, Acquire, Release};
use std::thread;

static mut DATA: String = String::new();
static LOCKED: AtomicBool = AtomicBool::new(false);

#[allow(static_mut_refs)]
fn f() {
if LOCKED.swap(true, Acquire) == false {
// Safety: We hold the exclusive lock, so nothing else is accessing DATA.
unsafe { DATA.push('!') };
LOCKED.store(false, Release);
}
}

fn main() {
thread::scope(|s| {
for _ in 0..100 {
s.spawn(f);
}
});
}

