use serde::Deserialize;

#[derive(Debug, PartialEq, Eq, Deserialize)]
pub struct Flags {
    pending: bool,
    flagged: bool,
    note_locked: bool,
    status_locked: bool,
    rating_locked: bool,
    deleted: bool
}
