use serde::Deserialize;

use crate::api::supplement::IdType;


#[derive(Debug, Deserialize)]
pub struct Relations {
    parent_id: Option<IdType>,
    has_children: bool,
    has_active_children: bool,
    children: Vec<IdType>
}
