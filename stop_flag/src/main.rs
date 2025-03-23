use std::sync::atomic::{AtomicBool, Ordering::Relaxed};
use std::thread;
use std::io::stdin;

static STOP: AtomicBool = AtomicBool::new(false);

fn some_work() {
    println!("İş davam edir...");
    thread::sleep(std::time::Duration::from_millis(100)); // İş prosesini simulyasiya edir
}

fn main() {
    // Fon iş parçacığını işə salırıq
    let background_thread = thread::spawn(|| {
        while !STOP.load(Relaxed) {
            some_work();
        }
        println!("İş parçacığı dayandı!");
    });


    println!("dsds");
    // Əsas iş parçacığı istifadəçi əmrlərini oxuyur
    for line in stdin().lines() {
        match line.unwrap().as_str() {
            "help" => println!("Əmrlər: help, stop"),
            "stop" => break, // Əgər istifadəçi "stop" daxil edərsə, döngü dayanır
            cmd => println!("Naməlum əmr: {cmd:?}"),
        }
    }

    // Fon iş parçacığını dayandırmaq üçün STOP bayrağını `true` edirik
    STOP.store(true, Relaxed);

    // Fon iş parçacığının tam dayanmasını gözləyirik
    background_thread.join().unwrap();

    println!("Proqram başa çatdı!");
}