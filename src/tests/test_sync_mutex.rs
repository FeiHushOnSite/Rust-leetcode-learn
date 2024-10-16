use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

#[test]
pub fn test_sync_mutex() {
    // CREATE SHARE DATA
    let theater = Arc::new(Theater::new(10));

    let mut handlers = vec![];

    for _ in 0..5 {
        let theater_clone = Arc::clone(&theater);
        let handler = thread::spawn(move || {
            theater_clone.book_ticket();
        });
        handlers.push(handler);
    }

    for handle in handlers {
        handle.join().unwrap();
    }

    println!("Final ticket count: {}", theater.get_available_tickets());
}

struct Theater {
    available_tickets: Mutex<i32>
}

impl Theater {
    fn new(initial_tickets: i32) -> Self {
        Theater {
            available_tickets: Mutex::new(initial_tickets)
        }
    }

    fn book_ticket(&self) {
        let mut tickets = self.available_tickets.lock().unwrap();
        if *tickets > 0 {
            thread::sleep(Duration::from_millis(100));
            *tickets -= 1;
            println!("Ticket booked. Remaining tickets: {}", *tickets);
        } else {
            println!("Sorry, no more tickets available.")
        }
    }

    fn get_available_tickets(&self) -> i32 {
        *self.available_tickets.lock().unwrap()
    }
}

