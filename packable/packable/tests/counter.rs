// Copyright 2021-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use packable::{
    packer::{CounterPacker, Packer},
    unpacker::{CounterUnpacker, SliceUnpacker, Unpacker},
    Packable, PackableExt,
};

#[test]
fn option_counter() {
    let mut packer = CounterPacker::new(Vec::new());

    let value = Some(0x45u32);
    value.pack(&mut packer).unwrap();
    assert_eq!(value.packed_len(), packer.counter());
    assert_eq!(packer.counter(), packer.written_bytes().unwrap());

    let packer = packer.into_inner();
    let mut unpacker = CounterUnpacker::new(SliceUnpacker::new(packer.as_slice()));

    let unpacked_value = Option::<u32>::unpack::<_, true>(&mut unpacker, &mut ()).unwrap();
    assert_eq!(unpacked_value.packed_len(), unpacker.counter());
    assert_eq!(unpacker.counter(), unpacker.read_bytes().unwrap());
    assert_eq!(value, unpacked_value);
}
