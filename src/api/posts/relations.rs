use serde::Deserialize;

use crate::api::supplement::IdType;


#[derive(Debug, Deserialize)]
pub struct Relations {
    pub parent_id: Option<IdType>,
    pub has_children: bool,
    pub has_active_children: bool,
    pub children: Vec<IdType>
}
