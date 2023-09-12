use std::thread;

use rust_spin_lock::SpinLock;

fn main() {
    let value = SpinLock::new(0);

    thread::scope(|t| {
        t.spawn(|| {
            *value.lock() = 10;
        });

        t.spawn(|| {
            *value.lock() = 20;
        });
    });

    println!("{}", *value.lock());
}
