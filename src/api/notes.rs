// Copyright 2024 Ezra Alvarion
//
// Licensed under the Apache License, Version 2.0 <LICENSE-Apache or
// https://www.apache.org/licenses/LICENSE-2.0> or the BSD 2-Clause 
// License <LICENSE-BSD or https://opensource.org/license/bsd-2-clause/>,
// at Your option. This file may not be copied, modified, or distributed
// except according to those terms.

use {
    serde::Deserialize,
    time::OffsetDateTime,

    super::datetimeformat,
};

pub const NOTES_URL: &'static str = "notes.json";

#[derive(Debug, Deserialize)]
pub struct NoteObject {
    pub id: u32,
    #[serde(with = "datetimeformat")]
    pub created_at: OffsetDateTime,
    #[serde(with = "datetimeformat")]
    pub updated_at: OffsetDateTime,
    pub creator_id: u32,
    pub x: u16, // These (x, y) values are from the top left of the screen: no negatives needed
    pub y: u16, // Also, the current semi-implemented highest resolution screen is 16K, so u16 is enough
    pub width: u16,
    pub height: u16,
    pub version: u16, // There is no way someone will go out of their way to change a note 2^16 times just to overflow this
    pub is_active: bool,
    pub post_id: u32,
    pub body: String,
    pub creator_name: String
}

impl NoteObject {
    
}
