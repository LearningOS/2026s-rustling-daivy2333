use std::sync::mpsc;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

struct Queue {
    length: u32,
    first_half: Vec<u32>,
    second_half: Vec<u32>,
}

impl Queue {
    fn new() -> Self {
        Queue {
            length: 10,
            first_half: vec![1, 2, 3, 4, 5],
            second_half: vec![6, 7, 8, 9, 10],
        }
    }
}

fn send_tx(q: Queue, tx: mpsc::Sender<u32>) -> () {
    let qc = Arc::new(q);
    let qc1 = Arc::clone(&qc);
    let qc2 = Arc::clone(&qc);

    // Clone the sender for the first thread
    let tx1 = tx.clone();
    thread::spawn(move || {
        for val in &qc1.first_half {
            println!("sending {:?}", val);
            tx1.send(*val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    // Use the original sender (or clone it again) for the second thread
    // Since we only have two threads, using 'tx' directly here works because 
    // tx1 was a clone, leaving 'tx' still available in this scope.
    thread::spawn(move || {
        for val in &qc2.second_half {
            println!("sending {:?}", val);
            tx.send(*val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
    
    // Note: We must ensure 'tx' and 'tx1' are dropped only after threads are done 
    // if we were joining them, but here main waits on rx. 
    // However, strictly speaking, dropping the last sender closes the channel.
    // In this specific exercise structure, the senders stay alive until the function ends.
    // But wait! If send_tx returns, tx and tx1 are dropped. 
    // The threads are detached (spawned without joining). 
    // If the main thread receives all messages before the spawned threads finish sending, 
    // it might exit? No, the loop `for received in rx` waits until the channel is closed.
    // The channel closes only when ALL senders are dropped.
    // The senders are moved into the threads. They will be dropped when the threads finish.
    // So this logic holds up.
}

fn main() {
    let (tx, rx) = mpsc::channel();
    let queue = Queue::new();
    let queue_length = queue.length;

    send_tx(queue, tx);

    let mut total_received: u32 = 0;
    for received in rx {
        println!("Got: {}", received);
        total_received += 1;
    }

    println!("total numbers received: {}", total_received);
    assert_eq!(total_received, queue_length)
}