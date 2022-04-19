// Copyright 2021-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use packable::{packer::CounterPacker, unpacker::CounterUnpacker, Packable, PackableExt};

#[test]
fn option_counter() {
    let mut packer = CounterPacker::new(Vec::new());

    let value = Some(0x45u32);
    value.pack(&mut packer).unwrap();
    assert_eq!(value.packed_len(), packer.counter());

    let packer = packer.into_inner();
    let mut unpacker = CounterUnpacker::new(packer.as_slice());

    let unpacked_value = Option::<u32>::unpack::<_, true>(&mut unpacker).unwrap();
    assert_eq!(unpacked_value.packed_len(), unpacker.counter());
    assert_eq!(value, unpacked_value);
}
