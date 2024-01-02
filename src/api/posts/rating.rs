use serde::{Deserialize, de::Visitor};


#[derive(Debug, PartialEq, Eq)]
pub enum Rating {
    Safe,
    Questionable,
    Explicit
}

impl<'de> Deserialize<'de> for Rating {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de> {
        struct RatingVisitor;
        
        impl<'de> Visitor<'de> for RatingVisitor {
            type Value = Rating;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a string equal to either `s`, `q`, or `e`")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
                where
                    E: serde::de::Error, {
                match v {
                    "s" => Ok(Self::Value::Safe),
                    "q" => Ok(Self::Value::Questionable),
                    "e" => Ok(Self::Value::Explicit),
                    _ => Err(E::custom(format!("unexpected rating {v}")))
                }
            }
        }
        deserializer.deserialize_str(RatingVisitor)
    }
}

// TODO: impl Serialize for Rating
