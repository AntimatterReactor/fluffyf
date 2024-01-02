pub mod pools;
pub mod posts;
pub mod file;
pub mod tags;
pub mod notes;
pub mod favorites;
pub mod search;
pub mod traits;
pub mod supplement;

time::serde::format_description!(datetimeformat, OffsetDateTime, "[year]-[month]-[day]T[hour]:[minute]:[second].[subsecond digits:3][offset_hour sign:mandatory]:[offset_minute]");

const BASE_URL1: &'static str = "https://e621.net";
const BASE_URL2: &'static str = "https://e926.net";
