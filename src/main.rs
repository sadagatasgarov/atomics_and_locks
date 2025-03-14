use std::thread;
fn main() {
    let a = thread::spawn(f);
    let b = thread::spawn(f);
    println!("Hello from the main thread.");

    a.join().unwrap();
    b.join().unwrap();
}
fn f() {
    println!("Hello from another thread!");
    let id = thread::current().id();
    println!("This is my thread id: {id:?}");
}
