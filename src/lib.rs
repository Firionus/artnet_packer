//! ArtPacker creates packets to send DMX over Art-Net.
//!
//! It complies with the [Art-Net v4 specification](https://art-net.org.uk/resources/art-net-specification/), Revision 1.4dd 22/1/2017.
//! However, only the ArtDmx packet from page 45 is implemented.
//!
//! ArtPacker packs unchanged data only every 800 ms. As in the sACN standard, we call this behaviour suppression.
//! 
//! # Disclaimer
//! 
//! This is my first Rust project. The code is tested but may not be optimal. This library is not on crates.io. 
//! I am not sure if I will support this library in the future. 
//! However, if you have any concerns, feel free to open an issue. 
//! 
//! For a lower level Art-Net library, see https://github.com/Trangar/artnet_protocol. 
//!
//! # How to use
//!
//! Before you start your render loop, `use` the three relevant types this module provides. Then create a new `ArtPacker` that keeps
//! track of previously packed data:
//!
//! ```
//! use artnet_packer::{ArtPacker, PortAddress, DMXData};
//! let mut packer = ArtPacker::new();
//! ```
//!
//! At every tick of your render loop, you pass a `HashMap<PortAddress, DMXData>` into ArtPacker:
//!
//! ```
//! # use artnet_packer::{ArtPacker, PortAddress, DMXData};
//! # let mut packer = ArtPacker::new();
//! use std::collections::HashMap;
//! use std::convert::TryFrom;
//! fn my_render_function() -> HashMap<PortAddress, DMXData> {
//!    let mut output = HashMap::new();
//!    output.insert(PortAddress::from(0), DMXData::try_from(vec![255; 512]).unwrap());
//!    output
//! }
//! // loop
//! let input: HashMap<PortAddress, DMXData> = my_render_function();
//! let packets_to_send: HashMap<PortAddress, Vec<u8>> = packer.pack(&input);
//! ```
//!
//! The ArtDmx packets in the form of `Vec<u8>` should then be sent over UDP to the subscribers of the corresponding Port Address.
//!
//! You should call `ArtPacker.pack` and send the result at least every 4 seconds to comply with Art-Net specifications.
//!
//! # Credit
//!
//! Art-Net™ Designed by and Copyright Artistic Licence Holdings Ltd

use std::collections::{HashMap, HashSet};
#[allow(unused_imports)]
use std::convert::TryFrom;

mod types;
pub use types::DMXData;
pub use types::PortAddress;

mod suppressor;
use suppressor::Suppressor;

mod create_art_dmx_packet;
use create_art_dmx_packet::create_art_dmx_packet;

pub struct ArtPacker(HashMap<PortAddress, Suppressor>);

impl ArtPacker {
    pub fn new() -> Self {
        ArtPacker(HashMap::new())
    }
    pub fn pack(&mut self, input: &HashMap<PortAddress, DMXData>) -> HashMap<PortAddress, Vec<u8>> {
        //allocate output
        let mut output: HashMap<PortAddress, Vec<u8>> = HashMap::new();

        for input_universe in input.keys() {
            if !(self.0.contains_key(input_universe)) {
                //there is no suppressor for this input universe
                //create one
                self.0
                    .insert(*input_universe, Suppressor::new(&input[input_universe]));
            }
            // at this point we are guaranteed that a suppressor for input_universe exists
            // the unwrap therefore should never panic
            if self
                .0
                .get_mut(input_universe)
                .unwrap()
                .should_pack(&input[input_universe])
            //ask suppressor if we should pack
            {
                output.insert(
                    *input_universe,
                    create_art_dmx_packet(input_universe, &input[input_universe]),
                );
            }
        }
        let mut suppressor_universes_to_remove: HashSet<PortAddress> = HashSet::new();
        for suppressor_universe in self.0.keys() {
            if !(input.contains_key(&suppressor_universe)) {
                //there is a suppressor that is not used in the input
                //delete the unused suppressor
                suppressor_universes_to_remove.insert(*suppressor_universe);
            }
        }
        for suppressor_universe_to_remove in suppressor_universes_to_remove {
            self.0.remove(&suppressor_universe_to_remove);
        }

        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn art_packer_pack_simple_packet_and_suppress_and_keepalive_and_break_suppression_by_change_of_data(
    ) {
        //create new art packer
        let mut my_packer = ArtPacker::new();
        //create test data
        let mut test_data = HashMap::new();
        test_data.insert(PortAddress::from(0), DMXData::try_from(vec![255]).unwrap());
        //manually assemble test packet from test data
        let mut comparison = HashMap::new();
        comparison.insert(
            PortAddress::from(0),
            vec![
                65, 114, 116, 45, 78, 101, 116, 0, 0, 80, 0, 14, 0, 0, 0, 0, 0, 2, 255, 0,
            ],
        );
        //test for equality
        assert!(my_packer.pack(&test_data) == comparison);
        //does not pack again if the same
        assert!(HashMap::new() == my_packer.pack(&test_data));
        //now wait for keepalive
        use std::thread::sleep;
        use std::time;
        sleep(time::Duration::from_millis(900));
        assert!(my_packer.pack(&test_data) == comparison); //sequence number would break this test

        //change data
        test_data.insert(PortAddress::from(0), DMXData::try_from(vec![254]).unwrap());
        comparison.insert(
            PortAddress::from(0),
            vec![
                65, 114, 116, 45, 78, 101, 116, 0, 0, 80, 0, 14, 0, 0, 0, 0, 0, 2, 254, 0,
            ],
        );
        assert!(my_packer.pack(&test_data) == comparison);

        // remove suppressor
        let empty_data = HashMap::new();
        assert!(my_packer.pack(&empty_data) == HashMap::new());

        //no suppression should happen now
        assert!(my_packer.pack(&test_data) == comparison);
    }
}
