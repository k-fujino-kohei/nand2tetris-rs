use super::dff::DFF;
use crate::ch_1::{mux, Bit};

struct BitRegister {
    dff: DFF,
}

impl BitRegister {
    pub fn new(v: Bit) -> Self {
        Self { dff: DFF::new(v) }
    }

    pub fn sync(&mut self, input: Bit, load: Bit) -> Bit {
        let out = mux(self.dff.out(), input, load);
        self.dff.sync(out)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test {
        ($bit:expr, $(| $time:literal | $in:literal | $load:literal | $out:literal |)+) => {
            $(
                assert_eq!($bit.sync($in, $load), $out, "failed at time: {}", $time);
            )+
        };
    }

    #[test]
    fn test_bit_register() {
        let mut bit = BitRegister::new(0);
        test!(bit,
            | "0+"   |  0  |  0  |  0  |
            | "1"    |  0  |  0  |  0  |
            | "1+"   |  0  |  1  |  0  |
            | "2"    |  0  |  1  |  0  |
            | "2+"   |  1  |  0  |  0  |
            | "3"    |  1  |  0  |  0  |
            | "3+"   |  1  |  1  |  0  |
            | "4"    |  1  |  1  |  1  |
            | "4+"   |  0  |  0  |  1  |
            | "5"    |  0  |  0  |  1  |
            | "5+"   |  1  |  0  |  1  |
            | "6"    |  1  |  0  |  1  |
            | "6+"   |  0  |  1  |  1  |
            | "7"    |  0  |  1  |  0  |
            | "7+"   |  1  |  1  |  0  |
            | "8"    |  1  |  1  |  1  |
            | "8+"   |  0  |  0  |  1  |
            | "9"    |  0  |  0  |  1  |
            | "9+"   |  0  |  0  |  1  |
            | "10"   |  0  |  0  |  1  |
            | "10+"  |  0  |  0  |  1  |
            | "11"   |  0  |  0  |  1  |
            | "11+"  |  0  |  0  |  1  |
            | "12"   |  0  |  0  |  1  |
            | "12+"  |  0  |  0  |  1  |
            | "13"   |  0  |  0  |  1  |
            | "13+"  |  0  |  0  |  1  |
            | "14"   |  0  |  0  |  1  |
            | "14+"  |  0  |  0  |  1  |
            | "15"   |  0  |  0  |  1  |
            | "15+"  |  0  |  0  |  1  |
            | "16"   |  0  |  0  |  1  |
            | "16+"  |  0  |  0  |  1  |
            | "17"   |  0  |  0  |  1  |
            | "17+"  |  0  |  0  |  1  |
            | "18"   |  0  |  0  |  1  |
            | "18+"  |  0  |  0  |  1  |
            | "19"   |  0  |  0  |  1  |
            | "19+"  |  0  |  0  |  1  |
            | "20"   |  0  |  0  |  1  |
            | "20+"  |  0  |  0  |  1  |
            | "21"   |  0  |  0  |  1  |
            | "21+"  |  0  |  0  |  1  |
            | "22"   |  0  |  0  |  1  |
            | "22+"  |  0  |  0  |  1  |
            | "23"   |  0  |  0  |  1  |
            | "23+"  |  0  |  0  |  1  |
            | "24"   |  0  |  0  |  1  |
            | "24+"  |  0  |  0  |  1  |
            | "25"   |  0  |  0  |  1  |
            | "25+"  |  0  |  0  |  1  |
            | "26"   |  0  |  0  |  1  |
            | "26+"  |  0  |  0  |  1  |
            | "27"   |  0  |  0  |  1  |
            | "27+"  |  0  |  0  |  1  |
            | "28"   |  0  |  0  |  1  |
            | "28+"  |  0  |  0  |  1  |
            | "29"   |  0  |  0  |  1  |
            | "29+"  |  0  |  0  |  1  |
            | "30"   |  0  |  0  |  1  |
            | "30+"  |  0  |  0  |  1  |
            | "31"   |  0  |  0  |  1  |
            | "31+"  |  0  |  0  |  1  |
            | "32"   |  0  |  0  |  1  |
            | "32+"  |  0  |  0  |  1  |
            | "33"   |  0  |  0  |  1  |
            | "33+"  |  0  |  0  |  1  |
            | "34"   |  0  |  0  |  1  |
            | "34+"  |  0  |  0  |  1  |
            | "35"   |  0  |  0  |  1  |
            | "35+"  |  0  |  0  |  1  |
            | "36"   |  0  |  0  |  1  |
            | "36+"  |  0  |  0  |  1  |
            | "37"   |  0  |  0  |  1  |
            | "37+"  |  0  |  0  |  1  |
            | "38"   |  0  |  0  |  1  |
            | "38+"  |  0  |  0  |  1  |
            | "39"   |  0  |  0  |  1  |
            | "39+"  |  0  |  0  |  1  |
            | "40"   |  0  |  0  |  1  |
            | "40+"  |  0  |  0  |  1  |
            | "41"   |  0  |  0  |  1  |
            | "41+"  |  0  |  0  |  1  |
            | "42"   |  0  |  0  |  1  |
            | "42+"  |  0  |  0  |  1  |
            | "43"   |  0  |  0  |  1  |
            | "43+"  |  0  |  0  |  1  |
            | "44"   |  0  |  0  |  1  |
            | "44+"  |  0  |  0  |  1  |
            | "45"   |  0  |  0  |  1  |
            | "45+"  |  0  |  0  |  1  |
            | "46"   |  0  |  0  |  1  |
            | "46+"  |  0  |  0  |  1  |
            | "47"   |  0  |  0  |  1  |
            | "47+"  |  0  |  0  |  1  |
            | "48"   |  0  |  0  |  1  |
            | "48+"  |  0  |  0  |  1  |
            | "49"   |  0  |  0  |  1  |
            | "49+"  |  0  |  0  |  1  |
            | "50"   |  0  |  0  |  1  |
            | "50+"  |  0  |  0  |  1  |
            | "51"   |  0  |  0  |  1  |
            | "51+"  |  0  |  0  |  1  |
            | "52"   |  0  |  0  |  1  |
            | "52+"  |  0  |  0  |  1  |
            | "53"   |  0  |  0  |  1  |
            | "53+"  |  0  |  0  |  1  |
            | "54"   |  0  |  0  |  1  |
            | "54+"  |  0  |  0  |  1  |
            | "55"   |  0  |  0  |  1  |
            | "55+"  |  0  |  0  |  1  |
            | "56"   |  0  |  0  |  1  |
            | "56+"  |  0  |  0  |  1  |
            | "57"   |  0  |  0  |  1  |
            | "57+"  |  0  |  1  |  1  |
            | "58"   |  0  |  1  |  0  |
            | "58+"  |  1  |  0  |  0  |
            | "59"   |  1  |  0  |  0  |
            | "59+"  |  1  |  0  |  0  |
            | "60"   |  1  |  0  |  0  |
            | "60+"  |  1  |  0  |  0  |
            | "61"   |  1  |  0  |  0  |
            | "61+"  |  1  |  0  |  0  |
            | "62"   |  1  |  0  |  0  |
            | "62+"  |  1  |  0  |  0  |
            | "63"   |  1  |  0  |  0  |
            | "63+"  |  1  |  0  |  0  |
            | "64"   |  1  |  0  |  0  |
            | "64+"  |  1  |  0  |  0  |
            | "65"   |  1  |  0  |  0  |
            | "65+"  |  1  |  0  |  0  |
            | "66"   |  1  |  0  |  0  |
            | "66+"  |  1  |  0  |  0  |
            | "67"   |  1  |  0  |  0  |
            | "67+"  |  1  |  0  |  0  |
            | "68"   |  1  |  0  |  0  |
            | "68+"  |  1  |  0  |  0  |
            | "69"   |  1  |  0  |  0  |
            | "69+"  |  1  |  0  |  0  |
            | "70"   |  1  |  0  |  0  |
            | "70+"  |  1  |  0  |  0  |
            | "71"   |  1  |  0  |  0  |
            | "71+"  |  1  |  0  |  0  |
            | "72"   |  1  |  0  |  0  |
            | "72+"  |  1  |  0  |  0  |
            | "73"   |  1  |  0  |  0  |
            | "73+"  |  1  |  0  |  0  |
            | "74"   |  1  |  0  |  0  |
            | "74+"  |  1  |  0  |  0  |
            | "75"   |  1  |  0  |  0  |
            | "75+"  |  1  |  0  |  0  |
            | "76"   |  1  |  0  |  0  |
            | "76+"  |  1  |  0  |  0  |
            | "77"   |  1  |  0  |  0  |
            | "77+"  |  1  |  0  |  0  |
            | "78"   |  1  |  0  |  0  |
            | "78+"  |  1  |  0  |  0  |
            | "79"   |  1  |  0  |  0  |
            | "79+"  |  1  |  0  |  0  |
            | "80"   |  1  |  0  |  0  |
            | "80+"  |  1  |  0  |  0  |
            | "81"   |  1  |  0  |  0  |
            | "81+"  |  1  |  0  |  0  |
            | "82"   |  1  |  0  |  0  |
            | "82+"  |  1  |  0  |  0  |
            | "83"   |  1  |  0  |  0  |
            | "83+"  |  1  |  0  |  0  |
            | "84"   |  1  |  0  |  0  |
            | "84+"  |  1  |  0  |  0  |
            | "85"   |  1  |  0  |  0  |
            | "85+"  |  1  |  0  |  0  |
            | "86"   |  1  |  0  |  0  |
            | "86+"  |  1  |  0  |  0  |
            | "87"   |  1  |  0  |  0  |
            | "87+"  |  1  |  0  |  0  |
            | "88"   |  1  |  0  |  0  |
            | "88+"  |  1  |  0  |  0  |
            | "89"   |  1  |  0  |  0  |
            | "89+"  |  1  |  0  |  0  |
            | "90"   |  1  |  0  |  0  |
            | "90+"  |  1  |  0  |  0  |
            | "91"   |  1  |  0  |  0  |
            | "91+"  |  1  |  0  |  0  |
            | "92"   |  1  |  0  |  0  |
            | "92+"  |  1  |  0  |  0  |
            | "93"   |  1  |  0  |  0  |
            | "93+"  |  1  |  0  |  0  |
            | "94"   |  1  |  0  |  0  |
            | "94+"  |  1  |  0  |  0  |
            | "95"   |  1  |  0  |  0  |
            | "95+"  |  1  |  0  |  0  |
            | "96"   |  1  |  0  |  0  |
            | "96+"  |  1  |  0  |  0  |
            | "97"   |  1  |  0  |  0  |
            | "97+"  |  1  |  0  |  0  |
            | "98"   |  1  |  0  |  0  |
            | "98+"  |  1  |  0  |  0  |
            | "99"   |  1  |  0  |  0  |
            | "99+"  |  1  |  0  |  0  |
            | "100"  |  1  |  0  |  0  |
            | "100+" |  1  |  0  |  0  |
            | "101"  |  1  |  0  |  0  |
            | "101+" |  1  |  0  |  0  |
            | "102"  |  1  |  0  |  0  |
            | "102+" |  1  |  0  |  0  |
            | "103"  |  1  |  0  |  0  |
            | "103+" |  1  |  0  |  0  |
            | "104"  |  1  |  0  |  0  |
            | "104+" |  1  |  0  |  0  |
            | "105"  |  1  |  0  |  0  |
            | "105+" |  1  |  0  |  0  |
            | "106"  |  1  |  0  |  0  |
            | "106+" |  1  |  0  |  0  |
            | "107"  |  1  |  0  |  0  |
        );
    }
}
