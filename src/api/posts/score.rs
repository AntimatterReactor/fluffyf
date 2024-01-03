use std::cmp::Ordering;

use serde::Deserialize;

#[derive(Debug, Eq, Deserialize)]
pub struct Score {
    pub up: i32,   // As of 2023, the most upvoted post [2848682] is at +21425 raw whereas
    pub down: i32, // the most downvoted post (don't view, seriously) [378180] is at -7033 raw
    pub total: i32
}

impl Ord for Score {
    fn cmp(&self, other: &Self) -> Ordering {
        self.total.cmp(&other.total)
    }
}

impl PartialOrd for Score {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Score {
    fn eq(&self, other: &Self) -> bool {
        self.total == other.total
    }
}

impl Score {
    fn count_total(&mut self) -> i32 {
        self.total = self.up + self.down;
        self.total
    }
}
