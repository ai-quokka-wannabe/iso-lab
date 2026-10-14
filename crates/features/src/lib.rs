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
}
