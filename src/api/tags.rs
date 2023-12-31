type TagType = Option<Vec<String>>;

#[derive(Debug, PartialEq)]
pub struct TagObject {
    general: TagType,
    artist: TagType,
    copyright: TagType,
    character: TagType,
    species: TagType,
    invalid: TagType,
    meta: TagType,
    lore: TagType,
    locked: TagType,
}

impl TagObject {
    
}
