use std::collections::BTreeMap;

#[test]
pub fn test_calendar_two() {}

fn calendar_two() {}

struct Calendar {
    map: BTreeMap<i32, i32>,
}

impl Calendar {
    fn new() -> Self {
        Self {
            map: Default::default(),
        }
    }

    fn book(&mut self, start: i32, end: i32) -> bool {
        *self.map.entry(start).or_default() += 1;
        *self.map.entry(end).or_default() -= 1;
        if self.map.iter().fold((0, false), |(acc, ok), (_, v)| (acc + v, ok || acc + v > 2)).1 {
            *self.map.entry(start).or_default() -= 1;
            *self.map.entry(end).or_default() += 1;
            false
        } else {
            true
        }
    }
}
