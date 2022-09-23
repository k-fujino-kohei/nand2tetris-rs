#[macro_use]
extern crate serde;

pub mod ch_3;

use csv::{ReaderBuilder, Trim};
use serde::{de::DeserializeOwned, Deserialize, Deserializer};

pub(crate) fn read_cmp<Record: DeserializeOwned>(cmp: &str) -> Vec<Record> {
    let mut reader = ReaderBuilder::new()
        .delimiter(b'|')
        .trim(Trim::All)
        .from_reader(cmp.as_bytes());
    let records = reader
        .deserialize::<Record>()
        .collect::<Result<Vec<_>, _>>()
        .unwrap();
    debug_assert!(!records.is_empty());
    records
}

#[allow(unused)]
macro_rules! bit {
    (@$digit:literal, $bin:expr) => {{
        let mut r: [u8; $digit] = [0; $digit];
        $bin.chars()
            .enumerate()
            .for_each(|(i, b)| r[i] = b.to_string().parse::<u8>().unwrap());
        r
    }};
}

fn bit3(num: impl std::fmt::Binary) -> [u8; 3] {
    let str = format!("{:>03b}", num);
    bit!(@3, str)
}
fn bit6(num: impl std::fmt::Binary) -> [u8; 6] {
    let str = format!("{:>06b}", num);
    bit!(@6, str)
}
fn bit16(num: impl std::fmt::Binary) -> [u8; 16] {
    let str = format!("{:>016b}", num);
    bit!(@16, str)
}

pub(crate) fn de_bit3<'de, D>(deserializer: D) -> Result<[u8; 3], D::Error>
where
    D: Deserializer<'de>,
{
    Ok(bit3(u8::deserialize(deserializer)?))
}
pub(crate) fn de_bit6<'de, D>(deserializer: D) -> Result<[u8; 6], D::Error>
where
    D: Deserializer<'de>,
{
    Ok(bit6(u8::deserialize(deserializer)?))
}

pub(crate) fn de_bit16<'de, D>(deserializer: D) -> Result<[u8; 16], D::Error>
where
    D: Deserializer<'de>,
{
    Ok(bit16(i16::deserialize(deserializer)?))
}
