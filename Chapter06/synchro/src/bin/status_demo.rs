extern crate synchro;

use std::sync::Arc;
use std::{thread, time};
use synchro::Semaphore;

const THRS: usize = 4;
static mut COUNTS: &mut [u64] = &mut [0; THRS];
static mut STATUS: &mut [bool] = &mut [false; THRS];

fn worker(id: usize, gate: Arc<Semaphore>) {
    unsafe {
        loop {
            gate.wait();
            STATUS[id] = true;
            COUNTS[id] += 1;
            STATUS[id] = false;
            gate.signal();
        }
    }
}

fn main() {
    let semaphore = Arc::new(Semaphore::new(2));

    for i in 0..THRS {
        let semaphore = Arc::clone(&semaphore);
        thread::spawn(move || worker(i, semaphore));
    }

    let mut counts: [u64; THRS] = [0; THRS];
    loop {
        unsafe {
            thread::sleep(time::Duration::from_millis(1_000));
            print!("|");
            for i in 0..THRS {
                print!(" {:>5}; {:010}/sec |", STATUS[i], COUNTS[i] - counts[i]);
                counts[i] = COUNTS[i];
            }
            println!();
        }
    }
}
