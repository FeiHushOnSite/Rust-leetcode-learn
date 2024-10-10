use std::collections::BinaryHeap;

#[test]
pub fn test_seat_manager() {
    let n = 9;
    let collect = SeatManager::new(n);
    println!("res {:?}", collect)
}

#[derive(Debug)]
struct SeatManager {
    seats: BinaryHeap<std::cmp::Reverse<i32>>,
}

/// `&Self` means the method takes an immutable reference.
/// if you need a mutable reference, change it to `&mut self` instead

impl SeatManager {

    fn new(n: i32) -> Self {
        Self {
            seats: (1..=n).map(|x| std::cmp::Reverse(x)).collect(),
        }
    }

    fn reserve(&mut self) -> i32 {
        self.seats.pop().unwrap().0
    }

    fn unreserve(&mut self, seat_number: i32) {
        self.seats.push(std::cmp::Reverse(seat_number));
    }
}
