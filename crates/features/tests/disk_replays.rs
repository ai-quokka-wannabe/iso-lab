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

//! The lab opens a Disk through Master Control's crate (STATE.md item 3 (d2a)): the wire's one
//! implementation, `liblink.so`, loaded from beside this test executable, where Master Control's
//! build script places it. A tiny Disk is written here through `record_open`, as Master Control's
//! own suite writes one, and read back through `replay_open` - the same calls the Disk reader of
//! (d2b) will make, so what this test proves is that the lab's build has the wire and can read a
//! Disk through it, frame by frame, bit for bit.

use master_control::link_dll::{CreatureState, LNK_OK, LNK_PEER_CLOSED, LinkDll, Message, SegmentPose, TRAILING_SEGMENTS_MAX, TickStateHeader};
use master_control::physics::{TICK_SECONDS, world_definition};
use std::path::PathBuf;

/// The Disk's first tick: not zero, so the header's start is told apart from a default.
const START_TICK: u64 = 10;
/// How many TICK_STATE frames the Disk holds.
const TICKS: u64 = 5;
/// One body's row at the `index`th recorded tick: every field set from the index, so a row
/// replayed at the wrong tick, or a field dropped on the way, cannot match.
fn recorded_row(index: u64) -> CreatureState {
    let step = index as f32;
    let mut segments = [SegmentPose::default(); TRAILING_SEGMENTS_MAX];
    for (joint, segment) in segments.iter_mut().enumerate() {
        let back = (joint + 1) as f32;
        *segment = SegmentPose {
            position: [0.25 * step, 0.5, -0.1 * back],
            yaw: 0.01 * back * step,
            pitch: -0.001 * back,
        };
    }
    CreatureState {
        creature_id: 7,
        position: [0.25 * step, 0.5, 0.0],
        yaw: 0.1 * step,
        pitch: 0.0,
        velocity: [8.0, 0.0, 0.0],
        yaw_rate: 0.1 / TICK_SECONDS,
        vocalisation: 0.0,
        segment_count: 8,
        segments,
    }
}

/// A directory of this test's own under cargo's target, removed on drop whether or not the
/// test passed.
struct Scratch(PathBuf);

impl Scratch {
    fn new(name: &str) -> Scratch {
        let directory = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join(format!("disk-replays-{name}-{}", std::process::id()));
        std::fs::create_dir_all(&directory).expect("a scratch directory under target/");
        Scratch(directory)
    }
}

impl Drop for Scratch {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

#[test]
fn a_disk_written_through_the_wire_replays_its_frames_in_order_bit_for_bit() {
    let wire = LinkDll::beside_executable().expect("liblink.so beside the test executable, where master-control's build.rs puts it");
    let fingerprint = wire.world_fingerprint(&world_definition());
    let scratch = Scratch::new("tiny");
    let disk = scratch.0.join("tiny.disk");

    let recorded: Vec<CreatureState> = (0..TICKS).map(recorded_row).collect();
    {
        let mut recorder = wire
            .record_open(&disk, fingerprint, START_TICK, TICK_SECONDS, 0.9, 0)
            .expect("the Disk opens for writing");
        for (index, row) in recorded.iter().enumerate() {
            let header = TickStateHeader {
                tick: START_TICK + index as u64,
                creature_count: 1,
                food_count: 0,
            };
            assert_eq!(
                recorder.send_tick_state(&header, std::slice::from_ref(row), &[]),
                LNK_OK,
                "the wire takes a TICK_STATE of one body at tick {}",
                header.tick
            );
        }
        assert_eq!(recorder.flush(), Ok(true), "a file takes everything at once");
        // Dropping the recorder writes BYE and closes the file, as a world's end does.
    }
    assert!(disk.is_file(), "the Disk is a file at {}", disk.display());

    // Another world's Disk is refused: the wire judges the world fingerprint at the door.
    let other_world = wire.replay_open(&disk, fingerprint.wrapping_add(1));
    assert!(other_world.is_err(), "a Disk of another world must be refused, not read");
    drop(other_world);

    let (mut replay, welcome) = wire.replay_open(&disk, fingerprint).expect("the Disk opens for reading in the world it was made in");
    assert_eq!(welcome.world_fingerprint, fingerprint);
    assert_eq!(welcome.current_tick, START_TICK, "the header's start tick");
    assert_eq!(welcome.nominal_dt_seconds.to_bits(), TICK_SECONDS.to_bits(), "the header's tick length");

    let mut frames: Vec<(u64, Vec<CreatureState>)> = Vec::new();
    let mut ended_with_bye = false;
    let mut polls_after_bye = 0;
    loop {
        match replay.poll() {
            Ok(Some(Message::TickState { header, states, foods })) => {
                assert!(!ended_with_bye, "nothing follows BYE on a Disk, but tick {} did", header.tick);
                assert_eq!(header.creature_count as usize, states.len());
                assert!(foods.is_empty(), "no food was recorded");
                frames.push((header.tick, states));
            }
            Ok(Some(Message::Bye)) => ended_with_bye = true,
            Ok(Some(other)) => panic!("the Disk holds only TICK_STATE frames and BYE, not {other:?}"),
            Ok(None) => {
                if ended_with_bye {
                    polls_after_bye += 1;
                    assert!(polls_after_bye < 1_000, "the file's end never arrived after BYE");
                }
            }
            Err(LNK_PEER_CLOSED) => break,
            Err(status) => panic!("the replay failed with status {status}"),
        }
    }
    assert!(ended_with_bye, "the Disk ends with BYE as a world does");

    let ticks: Vec<u64> = frames.iter().map(|(tick, _)| *tick).collect();
    let expected_ticks: Vec<u64> = (START_TICK..START_TICK + TICKS).collect();
    assert_eq!(ticks, expected_ticks, "every tick, once, in order");
    for ((tick, states), row) in frames.iter().zip(recorded.iter()) {
        assert_eq!(states.len(), 1, "one body at tick {tick}");
        let replayed = &states[0];
        assert_eq!(replayed, row, "the row at tick {tick} replays as recorded");
        // PartialEq on f32 would pass -0.0 for 0.0; the Disk is held to the bit.
        assert_eq!(replayed.position.map(f32::to_bits), row.position.map(f32::to_bits));
        assert_eq!(replayed.yaw.to_bits(), row.yaw.to_bits());
        for (replayed_segment, segment) in replayed.segments.iter().zip(row.segments.iter()) {
            assert_eq!(replayed_segment.position.map(f32::to_bits), segment.position.map(f32::to_bits));
            assert_eq!(replayed_segment.yaw.to_bits(), segment.yaw.to_bits());
            assert_eq!(replayed_segment.pitch.to_bits(), segment.pitch.to_bits());
        }
    }
}
