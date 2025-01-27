use serde::de::Deserializer;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

use crate::models::RedditResponse;

#[derive(Debug, PartialEq)]
pub enum LazyResponse {
    Raw(Value), // store the raw object
    Parsed(RedditResponse),
}

impl LazyResponse {
    pub fn get(&mut self) -> eyre::Result<&RedditResponse> {
        match self {
            LazyResponse::Parsed(r) => {
                // Already parsed, just return the reference
                Ok(r)
            }
            LazyResponse::Raw(v) => {
                // 1) Take the raw Value out of `v`
                let value = std::mem::take(v);

                // 2) Convert it to a RedditResponse
                let response: RedditResponse = serde_json::from_value(value)?;

                // 3) Store that newly parsed response in `self`
                *self = LazyResponse::Parsed(response);

                // 4) Now match again on `self` so we can reference the newly
                //    stored data, which lives as long as `self` does.
                match self {
                    LazyResponse::Parsed(r) => Ok(r),
                    LazyResponse::Raw(_) => unreachable!("We just set it to Parsed"),
                }
            }
        }
    }
}

impl Serialize for LazyResponse {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            LazyResponse::Raw(v) => v.serialize(serializer),
            LazyResponse::Parsed(r) => r.serialize(serializer),
        }
    }
}

impl<'de> Deserialize<'de> for LazyResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let val = Value::deserialize(deserializer)?;
        Ok(LazyResponse::Raw(val))
    }
}
