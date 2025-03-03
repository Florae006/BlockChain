// 序列化和反序列化

use bincode;
use serde::{Deserialize, Serialize};
use std::io::{self};

pub fn serialize_blockchain<T>(blockchain: &T) -> Result<Vec<u8>, io::Error>
where
    T: Serialize,
{
    let encoded: Vec<u8> = bincode::serialize(blockchain).unwrap();
    Ok(encoded)
}

pub fn deserialize_blockchain<'a, T>(bytes: &'a [u8]) -> Result<T, io::Error>
where
    T: Deserialize<'a>,
{
    let decoded: T = bincode::deserialize(bytes).unwrap();
    Ok(decoded)
}
