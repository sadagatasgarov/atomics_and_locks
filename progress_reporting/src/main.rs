use std::sync::atomic::{AtomicUsize, Ordering::Relaxed};
use std::thread;
use std::time::Duration;

fn process_item(i: usize) {
    println!("Element işlənir: {i}");
    thread::sleep(Duration::from_nanos(1000000000)); // İşləmə prosesini simulyasiya edir
}

fn main() {
    let num_done = AtomicUsize::new(0); // İşlənmiş elementlərin sayını saxlayan atomik dəyişən

    thread::scope(|s| {
        // Fon iş parçacığını işə salırıq
        s.spawn(|| {
            for i in 0..100 {
                process_item(i); // Elementi emal edirik
                num_done.store(i + 1, Relaxed); // Hazır olan element sayını yeniləyirik
            }
        });

        // Əsas iş parçacığı irəliləyiş haqqında məlumat verir
        loop {
            let n = num_done.load(Relaxed); // Hazır olan elementlərin sayını oxuyuruq
            if n == 100 { break; } // Əgər 100-ü tamamlamışıqsa, döngüdən çıxırıq
            println!("İş gedir... {n}/100 tamamlandı");
            thread::sleep(Duration::from_secs(1)); // Hər saniyə bir dəfə status yenilənir
        }
    });

    println!("İş tamamlandı!");
}
