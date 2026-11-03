// Copyright 2026 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Owned serialized maps with views borrowing their guest-owned storage.
use std::{
    convert::TryInto,
    fmt,
    str::{self, Utf8Error},
};

/// A malformed serialized map. No partial map is returned on failure.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MapDecodeError {
    Truncated,
    SizeOverflow,
    MissingTerminator,
    TrailingData,
    InvalidUtf8(Utf8Error),
}

impl fmt::Display for MapDecodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Truncated => f.write_str("truncated map"),
            Self::SizeOverflow => f.write_str("map length overflow"),
            Self::MissingTerminator => f.write_str("missing map field terminator"),
            Self::TrailingData => f.write_str("trailing map data"),
            Self::InvalidUtf8(error) => write!(f, "invalid map text: {}", error),
        }
    }
}
impl std::error::Error for MapDecodeError {}
impl From<Utf8Error> for MapDecodeError {
    fn from(error: Utf8Error) -> Self {
        Self::InvalidUtf8(error)
    }
}

/// Owns the guest allocation returned by a header-map host call.
///
/// Views remain valid across ordinary host calls, but cannot outlive this owner.
/// Values are opaque bytes; text validation is opt-in via [`Self::text_pairs`].
/// Names are validated as UTF-8 in both accessors. Order, duplicates, case,
/// empty fields, embedded NULs and whitespace are preserved.
///
/// ```compile_fail
/// use proxy_wasm::map::SerializedMap;
/// let pairs = {
///     let map = SerializedMap::from(vec![0, 0, 0, 0]);
///     map.pairs().unwrap()
/// };
/// println!("{:?}", pairs);
/// ```
#[derive(Debug, Default)]
pub struct SerializedMap {
    bytes: Vec<u8>,
}

impl From<Vec<u8>> for SerializedMap {
    /// Retains the buffer without copying. Validation occurs when requesting views.
    fn from(bytes: Vec<u8>) -> Self {
        Self { bytes }
    }
}

impl SerializedMap {
    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes
    }
    pub fn into_bytes(self) -> Vec<u8> {
        self.bytes
    }

    /// Validates the complete input and borrows names and byte-valued payloads.
    /// Empty buffers and a zero count both represent an empty map. Malformed
    /// input returns an error; no partial result is exposed. The pair vector is
    /// the only allocation, and its count is bounded by the input before reserve.
    pub fn pairs(&self) -> Result<Vec<(&str, &[u8])>, MapDecodeError> {
        self.decode(Ok)
    }

    /// Like [`Self::pairs`], additionally validating all values as UTF-8.
    /// Allocates one pair vector, without copying names or values.
    pub fn text_pairs(&self) -> Result<Vec<(&str, &str)>, MapDecodeError> {
        self.decode(|value| Ok(str::from_utf8(value)?))
    }

    fn decode<'a, V>(
        &'a self,
        value: impl Fn(&'a [u8]) -> Result<V, MapDecodeError>,
    ) -> Result<Vec<(&'a str, V)>, MapDecodeError> {
        let bytes = &self.bytes;
        if bytes.is_empty() {
            return Ok(Vec::new());
        }
        let count = read_u32(bytes, 0)?;
        let table_end = count
            .checked_mul(8)
            .and_then(|n| n.checked_add(4))
            .ok_or(MapDecodeError::SizeOverflow)?;
        // Every entry needs two terminators even when both fields are empty.
        let minimum = count
            .checked_mul(2)
            .and_then(|n| table_end.checked_add(n))
            .ok_or(MapDecodeError::SizeOverflow)?;
        if minimum > bytes.len() {
            return Err(MapDecodeError::Truncated);
        }
        let mut pairs = Vec::with_capacity(count);
        let mut cursor = table_end;
        for entry in 0..count {
            let key_len = read_u32(bytes, 4 + entry * 8)?;
            let value_len = read_u32(bytes, 8 + entry * 8)?;
            let key = field(bytes, &mut cursor, key_len)?;
            let payload = field(bytes, &mut cursor, value_len)?;
            pairs.push((str::from_utf8(key)?, value(payload)?));
        }
        if cursor != bytes.len() {
            return Err(MapDecodeError::TrailingData);
        }
        Ok(pairs)
    }
}

fn read_u32(bytes: &[u8], offset: usize) -> Result<usize, MapDecodeError> {
    let end = offset.checked_add(4).ok_or(MapDecodeError::SizeOverflow)?;
    let raw = bytes.get(offset..end).ok_or(MapDecodeError::Truncated)?;
    Ok(u32::from_le_bytes(raw.try_into().unwrap()) as usize)
}

fn field<'a>(bytes: &'a [u8], cursor: &mut usize, len: usize) -> Result<&'a [u8], MapDecodeError> {
    let end = cursor
        .checked_add(len)
        .ok_or(MapDecodeError::SizeOverflow)?;
    let next = end.checked_add(1).ok_or(MapDecodeError::SizeOverflow)?;
    let payload = bytes.get(*cursor..end).ok_or(MapDecodeError::Truncated)?;
    match bytes.get(end) {
        Some(0) => {}
        Some(_) => return Err(MapDecodeError::MissingTerminator),
        None => return Err(MapDecodeError::Truncated),
    }
    *cursor = next;
    Ok(payload)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn encode(pairs: &[(&[u8], &[u8])]) -> Vec<u8> {
        let mut bytes = (pairs.len() as u32).to_le_bytes().to_vec();
        for (key, value) in pairs {
            bytes.extend_from_slice(&(key.len() as u32).to_le_bytes());
            bytes.extend_from_slice(&(value.len() as u32).to_le_bytes());
        }
        for (key, value) in pairs {
            bytes.extend_from_slice(key);
            bytes.push(0);
            bytes.extend_from_slice(value);
            bytes.push(0);
        }
        bytes
    }

    #[test]
    fn borrowed_bytes_preserve_wire_content() {
        let input: &[(&[u8], &[u8])] = &[
            (b"X-Key", b" first "),
            (b"X-Key", b""),
            (b"", b"\0\xff"),
            ("é".as_bytes(), "你好".as_bytes()),
        ];
        let map = SerializedMap::from(encode(input));
        let pairs = map.pairs().unwrap();
        for ((key, value), (expected_key, expected_value)) in pairs.iter().zip(input) {
            assert_eq!(key.as_bytes(), *expected_key);
            assert_eq!(*value, *expected_value);
            // Actual views into the retained allocation, including empty fields.
            let range = map.as_bytes().as_ptr_range();
            assert!(range.contains(&key.as_ptr()));
            assert!(range.contains(&value.as_ptr()));
        }
        assert!(matches!(
            map.text_pairs(),
            Err(MapDecodeError::InvalidUtf8(_))
        ));
        let text = SerializedMap::from(encode(&[(b"key\0name", b"value\0text"), (b"", b"")]));
        assert_eq!(
            text.text_pairs().unwrap(),
            vec![("key\0name", "value\0text"), ("", "")]
        );
    }

    #[test]
    fn empty_maps_and_bad_text() {
        for bytes in [vec![], vec![0; 4]] {
            let map = SerializedMap::from(bytes);
            assert!(map.pairs().unwrap().is_empty());
            assert!(map.text_pairs().unwrap().is_empty());
        }
        let map = SerializedMap::from(encode(&[(b"\xff", b"value")]));
        assert!(matches!(map.pairs(), Err(MapDecodeError::InvalidUtf8(_))));
    }

    #[test]
    fn malformed_buffers_return_errors_without_panicking() {
        let valid = encode(&[(b"a", b"b"), (b"c", b"d")]);
        for length in 1..valid.len() {
            assert!(
                SerializedMap::from(valid[..length].to_vec())
                    .pairs()
                    .is_err(),
                "length {}",
                length
            );
        }
        for offset in [0, 4, 8, 12, 16] {
            let mut bad = valid.clone();
            bad[offset..offset + 4].copy_from_slice(&u32::MAX.to_le_bytes());
            assert!(SerializedMap::from(bad).pairs().is_err());
        }
        let mut bad = valid.clone();
        bad.push(0);
        assert_eq!(
            SerializedMap::from(bad).pairs(),
            Err(MapDecodeError::TrailingData)
        );
        for offset in [21, 23, 25, 27] {
            let mut bad = valid.clone();
            bad[offset] = 1;
            assert_eq!(
                SerializedMap::from(bad).pairs(),
                Err(MapDecodeError::MissingTerminator)
            );
        }
        assert_eq!(
            SerializedMap::from(vec![0; 5]).pairs(),
            Err(MapDecodeError::TrailingData)
        );
    }
}
