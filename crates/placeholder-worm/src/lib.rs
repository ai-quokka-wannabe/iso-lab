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

//! The lab's placeholder Program (STATE.md item 3): a chain of eight segments that does nothing but
//! undulate gently, so that the arena can be proven - a life lived, its Disk kept, its features
//! extracted - before any brain exists. It is not a brain and is never scored. It reads one sense,
//! the Grid's tick counter, and two facts the interface hands it, the tick's length and its body's
//! joint limit; nothing of the world reaches it.
//!
//! The gait is a travelling wave of one wavelength along the body, at 0.18 Hz with an amplitude of
//! 0.3 rad, a third of the Grid's 0.9 rad joint limit. The frequency is the dataset's own published
//! midbody crawling frequency, the median of the per-record medians of its magnitude in 23
//! block-1 recordings (docs/journal of 2026-10-14, *The Arena, Third Step*); the dataset's
//! definition of it has not been read, and a placeholder needs no more. The wave is a piecewise
//! parabola rather than a sine, so that only basic arithmetic is used and the gait is the same to
//! the bit on any machine.
//!
//! The C interface is tron-grid-lite's `libs/program-abi/include/tgl/tgl_program_abi.h`, ABI
//! version 10, mirrored in [`abi`] for the members this Program touches.

pub mod abi;
pub mod gait;
