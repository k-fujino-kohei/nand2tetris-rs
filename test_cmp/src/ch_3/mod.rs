use crate::{de_bit16, de_bit3, de_bit6, read_cmp};

macro_rules! impl_clock {
    ($record:ident) => {
        impl $record {
            pub fn clock(&self) -> bool {
                self.time.contains('+')
            }
        }
    };
}

pub mod bit {
    use super::*;

    #[derive(Deserialize)]
    pub struct Record {
        pub time: String,
        pub r#in: u8,
        pub load: u8,
        pub out: u8,
    }

    impl_clock!(Record);

    pub fn cmp() -> Vec<Record> {
        read_cmp::<Record>(include_str!("./Bit.cmp"))
    }
}

pub mod register {
    use super::*;

    #[derive(Deserialize)]
    pub struct Record {
        pub time: String,
        #[serde(deserialize_with = "de_bit16")]
        pub r#in: [u8; 16],
        pub load: u8,
        #[serde(deserialize_with = "de_bit16")]
        pub out: [u8; 16],
    }

    impl_clock!(Record);

    pub fn cmp() -> Vec<Record> {
        read_cmp::<Record>(include_str!("./Register.cmp"))
    }
}

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

    impl_clock!(Record);

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

    impl_clock!(Record);

    pub fn cmp() -> Vec<Record> {
        read_cmp::<Record>(include_str!("./RAM64.cmp"))
    }
}

pub mod pc {
    use super::*;

    #[derive(Deserialize)]
    pub struct Record {
        pub time: String,
        #[serde(deserialize_with = "de_bit16")]
        pub r#in: [u8; 16],
        pub reset: u8,
        pub load: u8,
        pub inc: u8,
        #[serde(deserialize_with = "de_bit16")]
        pub out: [u8; 16],
    }

    impl_clock!(Record);

    pub fn cmp() -> Vec<Record> {
        read_cmp::<Record>(include_str!("./PC.cmp"))
    }
}