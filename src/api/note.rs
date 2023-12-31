use time::Date;

#[derive(Debug, PartialEq)]
pub struct NoteObject {
    id: u32,
    created_at: Date,
    updated_at: Date,
    creator_id: u32,
    x: u16, // These (x, y) values are from the top left of the screen: no negatives needed
    y: u16, // Also, the current semi-implemented highest resolution screen is 16K, so u16 is enough
    width: u16,
    height: u16,
    version: u16, // There is no way someone will go out of their way to change a note 2^16 times just to overflow this
    is_active: bool,
    post_id: u32,
    body: String,
    creator_name: String
}

impl NoteObject {
    
}
