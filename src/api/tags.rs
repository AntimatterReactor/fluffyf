// Copyright 2024 Ezra Alvarion
//
// Licensed under the Apache License, Version 2.0 <LICENSE-Apache or
// https://www.apache.org/licenses/LICENSE-2.0> or the BSD 2-Clause 
// License <LICENSE-BSD or https://opensource.org/license/bsd-2-clause/>,
// at Your option. This file may not be copied, modified, or distributed
// except according to those terms.

use serde::Deserialize;

pub const TAGS_URL: &'static str = "tags.json";

#[derive(Debug, PartialEq, Eq)]
pub enum Kind {
    GENERAL = 0,
    ARTIST,
    COPYRIGHT,
    CHARACTER,
    SPECIES,
    INVALID,
    META,
    LORE = 8,
}

pub type TagType = Vec<String>;

#[derive(Debug, Deserialize)]
pub struct Tags {
    pub general: TagType,
    pub artist: TagType,
    pub copyright: TagType,
    pub character: TagType,
    pub species: TagType,
    pub invalid: TagType,
    pub meta: TagType,
    pub lore: TagType,
}

impl Tags {
    
}
