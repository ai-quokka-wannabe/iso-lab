/*
    Copyright (C) 2026 The AiQuokkaWannabe Authors <https://github.com/ai-quokka-wannabe/iso-lab>

    This program is free software: you can redistribute it and/or modify it under the terms of
    the GNU General Public License as published by the Free Software Foundation, either version
    3 of the License, or (at your option) any later version.

    This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY;
    without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.
    See the GNU General Public License for more details.

    You should have received a copy of the GNU General Public License along with this program.
    If not, see <https://www.gnu.org/licenses/>.
*/

//! Fetching the reference recordings, one checked file at a time.
//!
//! The Open Worm Movement Database publishes an MD5 for every file of every record
//! (data/README.md, *The Phase 0 Survey*). A file is used only once its bytes hash to that
//! value; this crate starts with that check, and the download itself comes next.

use md5::{Digest, Md5};
use std::fmt;
use std::io::{self, Read};

/// A file whose bytes do not hash to the checksum its record publishes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChecksumMismatch {
    /// The MD5 the record publishes, as lower-case hexadecimal.
    pub expected: String,
    /// The MD5 of the bytes read, as lower-case hexadecimal.
    pub actual: String,
}

impl fmt::Display for ChecksumMismatch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "MD5 mismatch: expected {}, got {}", self.expected, self.actual)
    }
}

impl std::error::Error for ChecksumMismatch {}

/// Why a file could not be accepted.
#[derive(Debug)]
pub enum VerifyError {
    /// The bytes could not be read.
    Io(io::Error),
    /// The bytes were read, and hash to something else.
    Mismatch(ChecksumMismatch),
}

impl fmt::Display for VerifyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(error) => write!(f, "reading the file failed: {error}"),
            Self::Mismatch(mismatch) => mismatch.fmt(f),
        }
    }
}

impl std::error::Error for VerifyError {}

/// The MD5 of everything `reader` yields, as lower-case hexadecimal, read in fixed-size
/// chunks so that a file of hundreds of megabytes never has to sit in memory whole.
pub fn md5_hex(mut reader: impl Read) -> io::Result<String> {
    let mut hasher = Md5::new();
    let mut buffer = [0_u8; 64 * 1024];
    loop {
        let read = match reader.read(&mut buffer) {
            Ok(0) => break,
            Ok(read) => read,
            Err(error) if error.kind() == io::ErrorKind::Interrupted => continue,
            Err(error) => return Err(error),
        };
        hasher.update(&buffer[..read]);
    }
    Ok(hasher.finalize().iter().map(|byte| format!("{byte:02x}")).collect())
}

/// Accepts the bytes of `reader` only if they hash to `expected`, the record's published MD5.
/// Zenodo writes its checksums as `md5:<hex>`; the prefix and the case of the hexadecimal are
/// both accepted, so the value can be passed as the metadata gives it.
pub fn verify_md5(reader: impl Read, expected: &str) -> Result<(), VerifyError> {
    let expected = expected.strip_prefix("md5:").unwrap_or(expected).to_ascii_lowercase();
    let actual = md5_hex(reader).map_err(VerifyError::Io)?;
    if actual == expected {
        Ok(())
    } else {
        Err(VerifyError::Mismatch(ChecksumMismatch { expected, actual }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // The test vectors are RFC 1321's own, appendix A.5 (https://www.rfc-editor.org/rfc/rfc1321).
    #[test]
    fn md5_matches_the_rfc_1321_test_suite() {
        let suite = [
            ("", "d41d8cd98f00b204e9800998ecf8427e"),
            ("a", "0cc175b9c0f1b6a831c399e269772661"),
            ("abc", "900150983cd24fb0d6963f7d28e17f72"),
            ("message digest", "f96b697d7cb7938d525a2f31aaf161d0"),
            ("abcdefghijklmnopqrstuvwxyz", "c3fcd3d76192e4007dfb496cca67e13b"),
        ];
        for (input, digest) in suite {
            assert_eq!(md5_hex(input.as_bytes()).unwrap(), digest, "input {input:?}");
        }
    }

    // Longer than one 64 KiB chunk, so the loop is crossed more than once.
    #[test]
    fn md5_is_the_same_across_chunk_boundaries() {
        let bytes = vec![0x5a_u8; 200_000];
        let whole = {
            let mut hasher = Md5::new();
            hasher.update(&bytes);
            hasher.finalize().iter().map(|byte| format!("{byte:02x}")).collect::<String>()
        };
        assert_eq!(md5_hex(bytes.as_slice()).unwrap(), whole);
    }

    #[test]
    fn verify_accepts_the_zenodo_form_in_either_case() {
        assert!(verify_md5("abc".as_bytes(), "md5:900150983cd24fb0d6963f7d28e17f72").is_ok());
        assert!(verify_md5("abc".as_bytes(), "900150983CD24FB0D6963F7D28E17F72").is_ok());
    }

    #[test]
    fn verify_rejects_other_bytes_and_says_what_it_saw() {
        match verify_md5("abd".as_bytes(), "md5:900150983cd24fb0d6963f7d28e17f72") {
            Err(VerifyError::Mismatch(mismatch)) => {
                assert_eq!(mismatch.expected, "900150983cd24fb0d6963f7d28e17f72");
                assert_ne!(mismatch.actual, mismatch.expected);
            }
            other => panic!("expected a mismatch, got {other:?}"),
        }
    }
}
