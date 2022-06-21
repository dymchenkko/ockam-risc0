#![no_std]
use serde::{Deserialize, Serialize};
#[derive(Deserialize, Serialize)]
pub struct Message<'a> {
    #[serde(with = "serde_bytes")]
    pub message: &'a[u8],
}