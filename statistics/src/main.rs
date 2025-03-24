use std::{
    sync::atomic::{AtomicU64, AtomicUsize},
    thread,
    time::{Duration, Instant},
};

fn main() {
    let num_done = &AtomicUsize::new(0);
    let total_time = &AtomicU64::new(0);
    let max_time = &AtomicU64::new(0);

    thread::scope(|s| {
        for t in 0..4 {
            s.spawn(move || {
                for i in 0..25 {
                    let start = Instant::now();
                    println!("{}", t * 25 + i);
                    let time_taken = start.elapsed().as_micros() as u64;
                    num_done.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                    total_time.fetch_add(time_taken, std::sync::atomic::Ordering::Relaxed);
                    max_time.fetch_max(time_taken, std::sync::atomic::Ordering::Relaxed);
                }
            });
        }
    });

    loop {
        let total_time =
            Duration::from_micros(total_time.load(std::sync::atomic::Ordering::Relaxed));
        let max_time = Duration::from_micros(max_time.load(std::sync::atomic::Ordering::Relaxed));

        let n = num_done.load(std::sync::atomic::Ordering::Relaxed);

        if n == 100 {
            break;
        }
        if n == 0 {
            println!("working nothing done yet")
        } else {
            println!(
                "working.. {n}/100 done, {:?} avarage, {:?} peak",
                total_time / n as u32,
                max_time
            );
        }
        thread::sleep(Duration::from_secs(1));
    }

    println!("Done!")
}
