# art_packer

ArtPacker creates packets to send DMX over Art-Net.

It complies with the [Art-Net v4 specification](https://art-net.org.uk/resources/art-net-specification/), Revision 1.4dd 22/1/2017.
However, only the ArtDmx packet from page 45 is implemented.

ArtPacker packs unchanged data only every 800 ms. As in the sACN standard, we call this behaviour suppression.

## How to use

Before you start your render loop, `use` the three relevant types this module provides. Then create a new `ArtPacker` that keeps
track of previously packed data:

```rust
use art_packer::{ArtPacker, PortAddress, DMXData};
let mut packer = ArtPacker::new();
```

At every tick of your render loop, you pass a `HashMap<PortAddress, DMXData>` into ArtPacker:

```rust
use std::collections::HashMap;
use std::convert::TryFrom;
fn my_render_function() -> HashMap<PortAddress, DMXData> {
   let mut output = HashMap::new();
   output.insert(PortAddress::from(0), DMXData::try_from(vec![255; 512]).unwrap());
   output
}
// loop
let input: HashMap<PortAddress, DMXData> = my_render_function();
let packets_to_send: HashMap<PortAddress, Vec<u8>> = packer.pack(&input);
```

The ArtDmx packets in the form of `Vec<u8>` should then be sent over UDP to the subscribers of the corresponding Port Address.

You should call `ArtPacker.pack` and send the result at least every 4 seconds to comply with Art-Net specifications.

## Credit

Art-Net™ Designed by and Copyright Artistic Licence Holdings Ltd
