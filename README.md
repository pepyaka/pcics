## PCI configuration space

PCI configuration space is the underlying way that the Conventional PCI, PCI-X and PCI Express perform auto configuration of the cards inserted into their bus.

This library implements decoding PCI configuration space and PCI Express extended configuration space.

## Design

The main purpose of this library is to represent configuration space data as a
hierarchical structures. Therefore, CPU and memory usage may not be optimal.

The library is divided into three parts:
- [PCI 3.0 Compatible Configuration Space Header](header)
- [PCI Configuration Space Capabilities](capabilities)
- [Extended Configuration Space Capabilities](extended_capabilities)

## Usage

```rust
# use pcics::{
#     DDR_OFFSET, ECS_OFFSET,
#     capabilities::{
#         bridge_subsystem_vendor_id::BridgeSubsystemVendorId, Capability, CapabilityKind,
#     },
#     extended_capabilities::{
#         vendor_specific_extended_capability::VendorSpecificExtendedCapability,
#         ExtendedCapability, ExtendedCapabilityKind,
#     },
#     Capabilities, ExtendedCapabilities, Header,
# };
let conf_space_data = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/tests/data/device/8086:2030/config"
));

let header = Header::try_from(&conf_space_data[..DDR_OFFSET]).unwrap();
assert_eq!((0x8086, 0x2030), (header.vendor_id, header.device_id));

let mut caps = Capabilities::new(&conf_space_data[DDR_OFFSET..ECS_OFFSET], &header);
let BridgeSubsystemVendorId {
    subsystem_vendor_id,
    ..
} = caps
    .find_map(|cap| {
        if let Ok(Capability {
            kind: CapabilityKind::BridgeSubsystemVendorId(ssvid),
            ..
        }) = cap
        {
            Some(ssvid)
        } else {
            None
        }
    })
    .unwrap();
assert_eq!(0x8086, subsystem_vendor_id);

let mut ecaps = ExtendedCapabilities::new(&conf_space_data[ECS_OFFSET..]);
let VendorSpecificExtendedCapability { header, .. } = ecaps
    .find_map(|ecap| {
        if let Ok(ExtendedCapability {
            kind: ExtendedCapabilityKind::VendorSpecificExtendedCapability(vsec),
            ..
        }) = ecap
        {
            Some(vsec)
        } else {
            None
        }
    })
    .unwrap();
assert_eq!(0x0c, header.vsec_length);
```
More detailed usage in modules descriptions
