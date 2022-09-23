use crate::{de_bit16, de_bit3, de_bit6, read_cmp};

pub mod ram8 {
    use super::*;

    #[derive(Deserialize)]
    pub struct Record {
        pub time: String,
        #[serde(deserialize_with = "de_bit16")]
        pub r#in: [u8; 16],
        pub load: u8,
        #[serde(deserialize_with = "de_bit3")]
        pub address: [u8; 3],
        #[serde(deserialize_with = "de_bit16")]
        pub out: [u8; 16],
    }

    pub fn cmp() -> Vec<Record> {
        read_cmp::<Record>(include_str!("./RAM8.cmp"))
    }
}

pub mod ram64 {
    use super::*;

    #[derive(Deserialize)]
    pub struct Record {
        pub time: String,
        #[serde(deserialize_with = "de_bit16")]
        pub r#in: [u8; 16],
        pub load: u8,
        #[serde(deserialize_with = "de_bit6")]
        pub address: [u8; 6],
        #[serde(deserialize_with = "de_bit16")]
        pub out: [u8; 16],
    }

    pub fn cmp() -> Vec<Record> {
        read_cmp::<Record>(include_str!("./RAM64.cmp"))
    }
}
