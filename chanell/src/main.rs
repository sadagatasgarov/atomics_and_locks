use std::{
    cell::UnsafeCell,
    marker::PhantomData,
    mem::MaybeUninit,
    sync::atomic::{AtomicBool, Ordering::*},
    thread::{self, Thread},
};

pub struct Channel<T> {
    message: UnsafeCell<MaybeUninit<T>>,
    ready: AtomicBool,
}

unsafe impl<T> Sync for Channel<T> where T: Send {}

pub struct Sender<'a, T> {
    channel: &'a Channel<T>,
    receiving_thread: Thread,
}

pub struct Receiver<'a, T> {
    channel: &'a Channel<T>,
    _no_send: PhantomData<*const ()>,
}

impl<T> Channel<T> {
    pub const fn new() -> Self {
        Self {
            message: UnsafeCell::new(MaybeUninit::uninit()),
            ready: AtomicBool::new(false),
        }
    }

    pub fn split<'a>(&'a mut self) -> (Sender<'a, T>, Receiver<'a, T>) {
        *self = Self::new(); // reset channel before split
        (
            Sender {
                channel: self,
                receiving_thread: thread::current(),
            },
            Receiver {
                channel: self,
                _no_send: PhantomData,
            },
        )
    }
}

impl<T> Sender<'_, T> {
    pub fn send(self, message: T) {
        unsafe { (*self.channel.message.get()).write(message) };
        self.channel.ready.store(true, Release);
        self.receiving_thread.unpark();
    }
}

impl<T> Receiver<'_, T> {
    pub fn receive(self) -> T {
        while !self.channel.ready.swap(false, Acquire) {
            thread::park();
        }
        unsafe { (*self.channel.message.get()).assume_init_read() }
    }
}

impl<T> Drop for Channel<T> {
    fn drop(&mut self) {
        if *self.ready.get_mut() {
            unsafe {
                self.message.get_mut().assume_init_drop();
            }
        }
    }
}

fn main() {
    let mut channel = Channel::new();

    thread::scope(|s| {
        let (sender, receiver) = channel.split();

        s.spawn(move || {
            sender.send("Salam dünya!");
        });

        let msg = receiver.receive();
        println!("Aldığım mesaj: {}", msg);
    });
}

// use std::{
//     collections::VecDeque,
//     sync::{Arc, Condvar, Mutex},
//     thread,
//     time::Duration,
// };

// /// İkili: həm `Mutex`, həm `Condvar`
// struct Shared<T> {
//     queue: Mutex<VecDeque<T>>,
//     available: Condvar,
// }

// /// Sender – Mesaj göndərmək üçün
// #[derive(Clone)]
// pub struct Sender<T> {
//     shared: Arc<Shared<T>>,
// }

// /// Receiver – Mesaj almaq üçün
// pub struct Receiver<T> {
//     shared: Arc<Shared<T>>,
// }

// /// Channel yarat – (sender, receiver) verir
// pub fn channel<T>() -> (Sender<T>, Receiver<T>) {
//     let shared = Arc::new(Shared {
//         queue: Mutex::new(VecDeque::new()),
//         available: Condvar::new(),
//     });

//     (
//         Sender {
//             shared: shared.clone(),
//         },
//         Receiver { shared },
//     )
// }

// impl<T> Sender<T> {
//     /// Mesaj göndər
//     pub fn send(&self, msg: T) {
//         let mut queue = self.shared.queue.lock().unwrap();
//         queue.push_back(msg);
//         // Mesaj gələn kimi `Condvar` vasitəsilə xəbər ver
//         self.shared.available.notify_one();
//     }
// }

// impl<T> Receiver<T> {
//     /// Bloklayaraq mesaj gözlə və al
//     pub fn recv(&self) -> T {
//         let mut queue = self.shared.queue.lock().unwrap();

//         loop {
//             // Əgər növbədə mesaj varsa, götür
//             if let Some(msg) = queue.pop_front() {
//                 return msg;
//             }

//             // Əks halda, yat və gözlə
//             queue = self.shared.available.wait(queue).unwrap();
//         }
//     }
// }

// fn main() {

// }
