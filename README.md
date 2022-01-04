## PCI configuration space

PCI configuration space is the underlying way that the Conventional PCI, PCI-X and PCI Express perform auto configuration of the cards inserted into their bus.

This library implements access to PCI configuration space and PCI Express extended configuration space.

## Design

The library is divided into three parts :
- PCI 3.0 Compatible Configuration Space Header
- PCI Configuration Space Capabilities
- Extended Configuration Space Capabilities

## Usage

```rust
# use pcics::{Header, Capabilities, ExtendedCapabilities, DDR_OFFSET, DDR_LENGTH, ECS_LENGTH};
# use byte::{ ctx::LE, BytesExt, };
let header_data = [0; DDR_OFFSET];
let header = Header::try_from(&header_data[..]).unwrap();

let ddr_data = [0; DDR_LENGTH];
let caps = Capabilities::new(&ddr_data, header.capabilities_pointer);

let ecs_data = [0; ECS_LENGTH];
let ecaps = ExtendedCapabilities::new(&ecs_data);
```
More detailed usage in module descriptions
