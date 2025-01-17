// rakrs
// Copyright (C) SOFe
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#[allow(unused_imports)]
use crate::prelude::*;

use std::io::{Error, ErrorKind, Read, Result, Write};

use rakrs_io::CanIo;

/// Handles the 16-byte magic sequence in RakNet protocol.
/// This is a marker type and does not take any memory.
#[derive(Clone, Debug)]
pub struct Magic;

const MAGIC_PAYLOAD: [u8; 16] = [
    0x00, 0xff, 0xff, 0x00, 0xfe, 0xfe, 0xfe, 0xfe, 0xfd, 0xfd, 0xfd, 0xfd, 0x12, 0x34, 0x56, 0x78,
];

impl CanIo for Magic {
    /// Writes the magic sequence to the stream.
    fn write<W: Write>(&self, mut w: W) -> Result<()> {
        w.write_all(&MAGIC_PAYLOAD)
    }

    /// Reads the magic sequence from the stream and validates it.
    fn read<R: Read>(mut r: R) -> Result<Self> {
        let mut payload = [0u8; 16];
        r.read_exact(&mut payload)?;
        if &payload == &MAGIC_PAYLOAD {
            Ok(Self)
        } else {
            Err(Error::new(ErrorKind::Other, "Magic payload mismatch"))
        }
    }
}
