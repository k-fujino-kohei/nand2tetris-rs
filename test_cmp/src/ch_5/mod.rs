use crate::{de_bit15, de_bit16, de_opt_bit16, de_raw_bit15, de_raw_bit16, read_cmp};

pub mod cpu {
    use super::*;

    #[derive(Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Record {
        pub time: String,
        #[serde(deserialize_with = "de_bit16")]
        pub in_m: [u8; 16],
        #[serde(deserialize_with = "de_raw_bit16")]
        pub instruction: [u8; 16],
        pub reset: u8,
        #[serde(deserialize_with = "de_opt_bit16")]
        pub out_m: Option<[u8; 16]>,
        pub write_m: u8,
        #[serde(deserialize_with = "de_bit15")]
        pub addre: [u8; 15],
        #[serde(deserialize_with = "de_bit15")]
        pub pc: [u8; 15],
        #[serde(deserialize_with = "de_bit16")]
        #[serde(rename = "DRegiste")]
        pub d_registe: [u8; 16],
    }

    impl_clock!(Record);

    pub fn cmp() -> Vec<Record> {
        read_cmp::<Record>(include_str!("./CPU.cmp"))
    }
}

pub mod memory {
    use super::*;

    #[derive(Deserialize)]
    pub struct Record {
        #[serde(deserialize_with = "de_bit16")]
        pub r#in: [u8; 16],
        pub load: u8,
        #[serde(deserialize_with = "de_raw_bit15")]
        pub address: [u8; 15],
        #[serde(deserialize_with = "de_bit16")]
        pub out: [u8; 16],
    }

    pub fn cmp() -> Vec<Record> {
        read_cmp::<Record>(include_str!("./Memory.cmp"))
    }
}
