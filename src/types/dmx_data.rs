use std::convert::TryFrom;

/// `DMXData` is a `Vec<u8>` with a length from 1 to 512. This is checked upon construction with `TryFrom`.
///
/// ```
/// use artnet_packer::DMXData;
/// use std::convert::TryInto;
/// let data: DMXData = vec![0,1,2,3,255].try_into().unwrap();
/// //could fail if length = 0 or over > 512
/// ```
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
pub struct DMXData(Vec<u8>);

impl TryFrom<Vec<u8>> for DMXData {
    type Error = String;
    fn try_from(input: Vec<u8>) -> Result<Self, Self::Error> {
        let channels = input.len();
        if channels > 512 || channels == 0 {
            Err(format!(
                "DMXData length must be from 1 to 512. Got {}",
                channels
            ))
        } else {
            Ok(DMXData(input))
        }
    }
}

impl From<DMXData> for Vec<u8> {
    fn from(input: DMXData) -> Vec<u8> {
        input.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn dmx_data_ensures_length_from_1_to_512() {
        assert!(
            DMXData::try_from(Vec::new()).is_err(),
            "0 size DMXData is no data and therefore not allowed"
        );
        assert!(
            DMXData::try_from(vec![0u8; 513]).is_err(),
            "size 513 is not allowable for DMXData"
        );
        assert!(
            DMXData::try_from(vec![0u8; 1_000]).is_err(),
            "size 1000 is not allowable for DMXData"
        );
        let _a = DMXData::try_from(vec![0u8; 512]);
        let _b = DMXData::try_from(vec![255u8; 128]);
    }

    #[test]
    fn u8_implements_from_dmx_data() {
        let a = vec![128u8, 243];
        let b = DMXData::try_from(a.clone()).unwrap();
        let c: Vec<u8> = b.into();
        assert!(a == c);
    }
}
