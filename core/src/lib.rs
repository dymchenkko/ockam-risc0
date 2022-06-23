#![no_std]
use serde::{Deserialize, Serialize};
#[derive(Deserialize, Serialize, Debug)]
pub struct Information<'a> {
    #[serde(with = "serde_bytes")]
    pub signature: &'a[u8],
    #[serde(with = "serde_bytes")]
    pub publickey: &'a[u8],
    #[serde(with = "serde_bytes")]
    pub message: &'a[u8],
}