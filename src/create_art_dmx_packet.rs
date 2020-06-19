use super::DMXData;
use super::PortAddress;
#[allow(unused_imports)]
use std::convert::TryInto;

//state-less part of ArtDMX packing
pub fn create_art_dmx_packet(universe: &PortAddress, dmx_data: &DMXData) -> Vec<u8> {
    let mut data: Vec<u8> = dmx_data.clone().into();
    //ensure data is of even length
    let is_odd = |x: usize| x & 1 == 1;
    if is_odd(data.len()) {
        data.push(0)
    }
    let length: u16 = data.len() as u16;
    vec![
        String::from("Art-Net").into_bytes(), // ID[0..7]
        vec![0x00],                           //ID[7] null termination
        vec![0x00, 0x50],                     //OpOutput=0x5000, low byte first
        vec![0x00, 0x0e],                     //ProtVer=14, high byte first
        vec![0x00],                           //Sequence, 0x00 to disable
        vec![0x00],                           //Physical, 0x00 to disable
        universe.to_le_bytes().to_vec(), //Port Address, two bytes with the highest bit 0, low byte first, use of Universe 0 is discouraged
        length.to_be_bytes().to_vec(), //Length of Data, must be even in range 2-512, two bytes, high byte first
        data,                          //some DMX data
    ]
    .concat()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn create_art_dmx_packet_works_for_a_small_example() {
        let a: DMXData = vec![1, 2, 3, 4, 0, 65, 255].try_into().unwrap();
        let u: PortAddress = 12.into();
        assert!(
            create_art_dmx_packet(&u, &a)
                == vec![
                    65, 114, 116, 45, 78, 101, 116, 0, 0, 80, 0, 14, 0, 0, 12, 0, 0, 8, 1, 2, 3, 4,
                    0, 65, 255, 0
                ]
        )
    }

    #[test]
    fn create_art_dmx_packet_works_for_a_big_example() {
        let a: DMXData = vec![255; 512].try_into().unwrap();
        let u: PortAddress = 32_767.try_into().unwrap();
        assert!(
            create_art_dmx_packet(&u, &a)
                == vec![
                    vec![65, 114, 116, 45, 78, 101, 116, 0, 0, 80, 0, 14, 0, 0,],
                    u.to_le_bytes().to_vec(),
                    512u16.to_be_bytes().to_vec(),
                    vec![255; 512]
                ]
                .concat()
        )
    }
}
