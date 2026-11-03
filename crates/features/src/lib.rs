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

//! Behavioural features of a worm, read the same way from the reference recordings and, later,
//! from the Grid's own Disks, so that both are measured by one piece of code.
//!
//! This first part reads a reference recording's skeletons: the Open Worm Movement Database's
//! features files hold them as `/coordinates/skeletons`, frames × 49 points × (x, y) in
//! micrometres, with frames that could not be skeletonised filled with NaN, and the frame rate
//! as the `fps` attribute of `/features_timeseries` (docs/journal of 2026-10-14, *Reading the
//! Features Files*).
//! The published per-frame feature table itself is not read: the lab computes its own measures
//! from the skeletons (docs/journal of 2026-10-14, *Reading the Features Files*).

pub mod eigenshapes;
pub mod gaps;
pub mod locomotion;
pub mod posture;
pub mod profile;
pub mod states;

use std::fmt;
use std::path::Path;

/// The skeletons of one recording.
#[derive(Debug, Clone, PartialEq)]
pub struct Skeletons {
    /// Frames per second of the recording.
    pub fps: f64,
    /// Points along each skeleton, head first.
    pub points: usize,
    /// The coordinates, frame after frame, `points` (x, y) pairs a frame, in micrometres.
    xy: Vec<f32>,
}

impl Skeletons {
    /// How many frames the recording has, skeletonised or not.
    pub fn frames(&self) -> usize {
        self.xy.len() / (2 * self.points)
    }

    /// Frame `index`'s skeleton as (x, y) pairs, or `None` when the frame has no skeleton (any
    /// coordinate is NaN or infinite) or does not exist.
    pub fn frame(&self, index: usize) -> Option<&[f32]> {
        let width = 2 * self.points;
        let frame = self.xy.get(index * width..(index + 1) * width)?;
        frame.iter().all(|value| value.is_finite()).then_some(frame)
    }

    /// The recording's first `frames` frames only, or all of it when it has fewer.
    pub fn first(&self, frames: usize) -> Skeletons {
        let width = 2 * self.points;
        Skeletons {
            fps: self.fps,
            points: self.points,
            xy: self.xy[..(frames * width).min(self.xy.len())].to_vec(),
        }
    }

    /// How many frames carry a whole skeleton.
    pub fn skeletonised(&self) -> usize {
        (0..self.frames()).filter(|&index| self.frame(index).is_some()).count()
    }
}

/// Why a recording could not be read.
#[derive(Debug)]
pub enum ReadError {
    /// The HDF5 library could not open the file or read an object.
    Hdf5(String),
    /// The file was read but does not have the expected shape or attributes.
    Format(String),
}

impl fmt::Display for ReadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Hdf5(message) => write!(f, "HDF5: {message}"),
            Self::Format(message) => write!(f, "unexpected format: {message}"),
        }
    }
}

impl std::error::Error for ReadError {}

const SKELETONS: &str = "/coordinates/skeletons";
const TIMESERIES: &str = "/features_timeseries";

/// Reads the skeletons and frame rate of a features file at `path`.
pub fn read_skeletons(path: &Path) -> Result<Skeletons, ReadError> {
    read_file(&hdf5_pure::File::open(path).map_err(|error| ReadError::Hdf5(error.to_string()))?)
}

fn read_file(file: &hdf5_pure::File) -> Result<Skeletons, ReadError> {
    let hdf5 = |error: hdf5_pure::Error| ReadError::Hdf5(error.to_string());
    let dataset = file.dataset(SKELETONS).map_err(hdf5)?;
    let shape = dataset.shape().map_err(hdf5)?;
    let [frames, points, 2] = shape[..] else {
        return Err(ReadError::Format(format!("{SKELETONS} has shape {shape:?}, not [frames, points, 2]")));
    };
    match dataset.dtype().map_err(hdf5)? {
        hdf5_pure::DType::F32 => {}
        other => return Err(ReadError::Format(format!("{SKELETONS} holds {other:?}, not 32-bit floats"))),
    }
    // read_f32 decodes the stored byte order; the type was checked above, so nothing is converted.
    let xy = dataset.read_f32().map_err(hdf5)?;
    let expected = frames * points * 2;
    if xy.len() as u64 != expected {
        return Err(ReadError::Format(format!("{SKELETONS} holds {} values, not {expected}", xy.len())));
    }
    let fps = match file.dataset(TIMESERIES).map_err(hdf5)?.attrs().map_err(hdf5)?.get("fps") {
        Some(hdf5_pure::AttrValue::F64(fps)) => *fps,
        Some(hdf5_pure::AttrValue::F32(fps)) => f64::from(*fps),
        other => return Err(ReadError::Format(format!("{TIMESERIES} has no numeric fps attribute ({other:?})"))),
    };
    if !(fps.is_finite() && fps > 0.0) {
        return Err(ReadError::Format(format!("the frame rate {fps} is not positive")));
    }
    Ok(Skeletons {
        fps,
        points: points as usize,
        xy,
    })
}

/// The frame index of a features file: for each row, the video frame it stands for and the id of
/// the skeleton it was built from (docs/journal of 2026-11-03, *The Gaps by Cause: the Plan,
/// Written Before the Numbers*).
#[derive(Debug, Clone, PartialEq)]
pub struct FrameIndex {
    /// The video frame of the first row; the rows then step by one frame each.
    pub first_frame: u64,
    /// Each row's skeleton id, or `None` where Tierpsy found no good skeleton (`-1`).
    pub skeleton_ids: Vec<Option<u32>>,
}

/// Reads the `timestamp` and `skeleton_id` columns of `/features_timeseries`, refusing a file whose
/// timestamps do not start at a whole frame and step by exactly one.
///
/// The dataset's records are decoded here rather than by `hdf5-pure`, whose shuffle filter refuses
/// a chunk whose length is not a whole number of records. These files' pipeline applies Fletcher-32
/// before the shuffle, so each chunk the shuffle sees is its records plus a four-byte checksum;
/// libhdf5's shuffle leaves such trailing bytes where they are, and so does `decode_chunk`. The
/// checksum itself is not verified: each whole file's MD5 was checked on download.
pub fn read_frame_index(path: &Path) -> Result<FrameIndex, ReadError> {
    let hdf5 = |error: hdf5_pure::Error| ReadError::Hdf5(error.to_string());
    let io = |error: std::io::Error| ReadError::Hdf5(error.to_string());
    let file = hdf5_pure::File::open(path).map_err(hdf5)?;
    let dataset = file.dataset(TIMESERIES).map_err(hdf5)?;
    let datatype = dataset.datatype().map_err(hdf5)?;
    let hdf5_pure::Datatype::Compound { size, .. } = &datatype else {
        return Err(ReadError::Format(format!("{TIMESERIES} is not a compound dataset")));
    };
    let size = *size as usize;
    let pipeline: Vec<u16> = dataset.filter_pipeline().iter().map(|filter| filter.id).collect();
    // Fletcher-32 (3), shuffle (2), deflate (1), in the order they were applied on writing.
    if pipeline != [3, 2, 1] {
        return Err(ReadError::Format(format!(
            "{TIMESERIES} has filters {pipeline:?}, not Fletcher-32, shuffle and deflate"
        )));
    }
    let rows = match dataset.shape().map_err(hdf5)?[..] {
        [rows] => rows as usize,
        ref shape => return Err(ReadError::Format(format!("{TIMESERIES} has shape {shape:?}"))),
    };
    let chunk_rows = match dataset.chunk_shape().map_err(hdf5)?.as_deref() {
        Some(&[chunk_rows]) if chunk_rows > 0 => chunk_rows as usize,
        other => return Err(ReadError::Format(format!("{TIMESERIES} has chunks {other:?}"))),
    };
    let mut raw = vec![0u8; rows * size];
    let mut covered = 0;
    let mut bytes = std::fs::File::open(path).map_err(io)?;
    for chunk in dataset.chunks().map_err(hdf5)? {
        if chunk.filter_mask != 0 {
            return Err(ReadError::Format(format!("a chunk of {TIMESERIES} skips filters ({:#x})", chunk.filter_mask)));
        }
        let mut stored = vec![0u8; chunk.storage_size as usize];
        std::io::Seek::seek(&mut bytes, std::io::SeekFrom::Start(chunk.address)).map_err(io)?;
        std::io::Read::read_exact(&mut bytes, &mut stored).map_err(io)?;
        let records = decode_chunk(&stored, chunk_rows, size)?;
        let first = chunk.offset.first().copied().unwrap_or(0) as usize;
        if first >= rows {
            continue;
        }
        let take = chunk_rows.min(rows - first) * size;
        raw[first * size..first * size + take].copy_from_slice(&records[..take]);
        covered += take;
    }
    if covered != raw.len() {
        return Err(ReadError::Format(format!("{TIMESERIES}'s chunks cover {covered} of {} bytes", raw.len())));
    }
    frame_index(&datatype, &raw, "timestamp", "skeleton_id")
}

/// One stored chunk of `rows` records of `size` bytes, inflated and unshuffled; the four bytes of
/// the Fletcher-32 checksum after the records are dropped.
fn decode_chunk(stored: &[u8], rows: usize, size: usize) -> Result<Vec<u8>, ReadError> {
    let mut inflated = Vec::new();
    std::io::Read::read_to_end(&mut flate2::read::ZlibDecoder::new(stored), &mut inflated).map_err(|error| ReadError::Hdf5(format!("inflate: {error}")))?;
    if inflated.len() != rows * size + 4 {
        return Err(ReadError::Format(format!(
            "a chunk inflates to {} bytes, not {rows} records of {size} and a checksum",
            inflated.len()
        )));
    }
    // Shuffled, byte k of every record lies together, record after record; the checksum follows.
    let mut records = vec![0u8; rows * size];
    for record in 0..rows {
        for byte in 0..size {
            records[record * size + byte] = inflated[byte * rows + record];
        }
    }
    Ok(records)
}

/// One little-endian 32-bit float field of every record of a compound dataset's raw bytes.
fn f32_field(datatype: &hdf5_pure::Datatype, raw: &[u8], name: &str) -> Result<Vec<f32>, ReadError> {
    let hdf5_pure::Datatype::Compound { size, members } = datatype else {
        return Err(ReadError::Format(format!("{TIMESERIES} is not a compound dataset")));
    };
    let member = members
        .iter()
        .find(|member| member.name == name)
        .ok_or_else(|| ReadError::Format(format!("{TIMESERIES} has no field {name}")))?;
    match &member.datatype {
        hdf5_pure::Datatype::FloatingPoint {
            size: 4,
            byte_order: hdf5_pure::DatatypeByteOrder::LittleEndian,
            ..
        } => {}
        other => return Err(ReadError::Format(format!("{TIMESERIES} field {name} is {other:?}, not a little-endian 32-bit float"))),
    }
    let (size, offset) = (*size as usize, member.byte_offset as usize);
    if size == 0 || !raw.len().is_multiple_of(size) || offset + 4 > size {
        return Err(ReadError::Format(format!("{TIMESERIES} has {} bytes in records of {size}", raw.len())));
    }
    Ok(raw
        .chunks_exact(size)
        .map(|record| f32::from_le_bytes([record[offset], record[offset + 1], record[offset + 2], record[offset + 3]]))
        .collect())
}

fn frame_index(datatype: &hdf5_pure::Datatype, raw: &[u8], timestamp: &str, skeleton_id: &str) -> Result<FrameIndex, ReadError> {
    let timestamps = f32_field(datatype, raw, timestamp)?;
    let ids = f32_field(datatype, raw, skeleton_id)?;
    let Some(&first) = timestamps.first() else {
        return Err(ReadError::Format(format!("{TIMESERIES} has no rows")));
    };
    // Frame numbers are whole and, in these files, far below 2^24, where every integer is an f32.
    if !(first >= 0.0 && first.fract() == 0.0 && first < 16_777_216.0) {
        return Err(ReadError::Format(format!("the first timestamp {first} is not a frame number")));
    }
    if let Some(row) = timestamps.windows(2).position(|pair| pair[1] - pair[0] != 1.0) {
        return Err(ReadError::Format(format!("the timestamp does not step by one frame at row {}", row + 1)));
    }
    let skeleton_ids = ids
        .iter()
        .map(|&id| match id {
            -1.0 => Ok(None),
            id if id >= 0.0 && id.fract() == 0.0 && id < 16_777_216.0 => Ok(Some(id as u32)),
            id => Err(ReadError::Format(format!("{id} is not a skeleton id"))),
        })
        .collect::<Result<Vec<_>, _>>()?;
    Ok(FrameIndex {
        first_frame: first as u64,
        skeleton_ids,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use hdf5_pure::{AttrValue, File, FileBuilder};

    /// A features file in miniature, written with the filters the real ones use (Fletcher-32,
    /// shuffle, deflate) on a chunked layout: three frames of three points, the middle one NaN.
    fn fixture(fps: Option<AttrValue>, shape: &[u64]) -> File {
        fixture_as(fps, shape, Stored::F32)
    }

    /// How the fixture stores its coordinates.
    enum Stored {
        F32,
        F64,
        I32,
    }

    fn fixture_as(fps: Option<AttrValue>, shape: &[u64], stored: Stored) -> File {
        let mut xy = vec![0.0_f32, 0.0, 1.0, 0.0, 2.0, 0.0];
        xy.extend([f32::NAN; 6]);
        xy.extend([5.0_f32, 5.0, 5.0, 6.0, 5.0, 7.0]);
        let mut chunks = shape.to_vec();
        chunks[0] = 1;
        let mut builder = FileBuilder::new();
        let mut coordinates = builder.create_group("coordinates");
        let skeletons = coordinates.create_dataset("skeletons");
        match stored {
            Stored::F32 => skeletons.with_f32_data(&xy),
            Stored::F64 => skeletons.with_f64_data(&xy.iter().map(|&value| f64::from(value)).collect::<Vec<_>>()),
            Stored::I32 => skeletons.with_i32_data(&xy.iter().map(|&value| if value.is_finite() { value as i32 } else { 0 }).collect::<Vec<_>>()),
        }
        .with_shape(shape)
        .with_chunks(&chunks)
        .with_fletcher32()
        .with_shuffle()
        .with_deflate(4);
        builder.add_group(coordinates.finish());
        let timeseries = builder.create_dataset("features_timeseries");
        timeseries.with_f32_data(&[0.0, 1.0, 2.0]);
        if let Some(fps) = fps {
            timeseries.set_attr("fps", fps);
        }
        File::from_bytes(builder.finish().unwrap()).unwrap()
    }

    #[test]
    fn reads_frames_points_and_the_frame_rate() {
        let skeletons = read_file(&fixture(Some(AttrValue::F64(30.0)), &[3, 3, 2])).unwrap();
        assert_eq!((skeletons.frames(), skeletons.points, skeletons.fps), (3, 3, 30.0));
        assert_eq!(skeletons.frame(0), Some(&[0.0, 0.0, 1.0, 0.0, 2.0, 0.0][..]));
        assert_eq!(skeletons.frame(1), None, "a NaN frame has no skeleton");
        assert_eq!(skeletons.frame(2), Some(&[5.0, 5.0, 5.0, 6.0, 5.0, 7.0][..]));
        assert_eq!(skeletons.frame(3), None, "past the end");
        assert_eq!(skeletons.skeletonised(), 2);
        let first = skeletons.first(2);
        assert_eq!((first.frames(), first.frame(0), first.frame(2)), (2, skeletons.frame(0), None));
        assert_eq!(skeletons.first(9).frames(), 3);
    }

    #[test]
    fn refuses_a_missing_frame_rate_or_a_wrong_shape() {
        assert!(matches!(read_file(&fixture(None, &[3, 3, 2])), Err(ReadError::Format(_))));
        assert!(matches!(read_file(&fixture(Some(AttrValue::F64(0.0)), &[3, 3, 2])), Err(ReadError::Format(_))));
        assert!(matches!(read_file(&fixture(Some(AttrValue::F64(30.0)), &[3, 6])), Err(ReadError::Format(_))));
        assert!(matches!(read_file(&fixture(Some(AttrValue::F64(30.0)), &[2, 3, 3])), Err(ReadError::Format(_))));
    }

    // Values of another stored type would be read as garbage if their bytes were taken as
    // 32-bit floats; the reader must refuse them instead.
    #[test]
    fn refuses_coordinates_that_are_not_32_bit_floats() {
        for stored in [Stored::F64, Stored::I32] {
            assert!(matches!(
                read_file(&fixture_as(Some(AttrValue::F64(30.0)), &[3, 3, 2], stored)),
                Err(ReadError::Format(_))
            ));
        }
    }

    /// A compound timeseries in miniature, as (timestamp, skeleton id) records; the builder names
    /// a tuple's fields "0" and "1".
    fn timeseries(records: &[(f32, f32)]) -> (hdf5_pure::Datatype, Vec<u8>) {
        let mut builder = FileBuilder::new();
        builder.create_dataset("features_timeseries").with_compound_values(records).unwrap();
        let file = File::from_bytes(builder.finish().unwrap()).unwrap();
        let dataset = file.dataset(TIMESERIES).unwrap();
        (dataset.datatype().unwrap(), dataset.read_raw().unwrap())
    }

    #[test]
    fn reads_the_first_frame_and_the_skeleton_ids() {
        let (datatype, raw) = timeseries(&[(7.0, 3.0), (8.0, -1.0), (9.0, 4.0)]);
        let index = frame_index(&datatype, &raw, "0", "1").unwrap();
        assert_eq!(index.first_frame, 7);
        assert_eq!(index.skeleton_ids, vec![Some(3), None, Some(4)]);
    }

    #[test]
    fn refuses_a_timestamp_that_skips_or_a_field_that_is_absent() {
        let (datatype, raw) = timeseries(&[(0.0, 0.0), (2.0, 1.0)]);
        assert!(matches!(frame_index(&datatype, &raw, "0", "1"), Err(ReadError::Format(_))));
        let (datatype, raw) = timeseries(&[(0.5, 0.0), (1.5, 1.0)]);
        assert!(matches!(frame_index(&datatype, &raw, "0", "1"), Err(ReadError::Format(_))));
        let (datatype, raw) = timeseries(&[(0.0, -2.0)]);
        assert!(matches!(frame_index(&datatype, &raw, "0", "1"), Err(ReadError::Format(_))));
        let (datatype, raw) = timeseries(&[(0.0, 0.0)]);
        assert!(matches!(frame_index(&datatype, &raw, "0", "timestamp"), Err(ReadError::Format(_))));
    }

    /// What libhdf5 stores for records under Fletcher-32, shuffle and deflate, the checksum's
    /// four bytes standing in as `0xAB` (they are not read).
    fn stored_chunk(records: &[u8], size: usize) -> Vec<u8> {
        let rows = records.len() / size;
        let mut shuffled: Vec<u8> = (0..size).flat_map(|byte| (0..rows).map(move |record| records[record * size + byte])).collect();
        shuffled.extend([0xAB; 4]);
        let mut encoder = flate2::write::ZlibEncoder::new(Vec::new(), flate2::Compression::default());
        std::io::Write::write_all(&mut encoder, &shuffled).unwrap();
        encoder.finish().unwrap()
    }

    #[test]
    fn a_chunk_is_inflated_and_unshuffled_past_its_checksum() {
        let records: Vec<u8> = (0..3 * 5).map(|value| value as u8).collect();
        assert_eq!(decode_chunk(&stored_chunk(&records, 5), 3, 5).unwrap(), records);
        assert!(matches!(decode_chunk(&stored_chunk(&records, 5), 4, 5), Err(ReadError::Format(_))));
        // Without its checksum the chunk is not what these files store, and is refused.
        let mut encoder = flate2::write::ZlibEncoder::new(Vec::new(), flate2::Compression::default());
        std::io::Write::write_all(&mut encoder, &records).unwrap();
        assert!(matches!(decode_chunk(&encoder.finish().unwrap(), 3, 5), Err(ReadError::Format(_))));
    }
}
