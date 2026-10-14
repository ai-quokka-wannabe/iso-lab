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

//! The Program's side of the Grid's C interface, ABI version 10: `#[repr(C)]` mirrors of the
//! structs this Program reads or writes, the vtable, and the one exported symbol. Every other
//! struct is reached only through a pointer it never follows.
//!
//! The boundary is the one place in the lab's Rust where `unsafe` is allowed, and only where a
//! pointer from the Grid is followed; each such place says why it is sound. No function here can
//! panic, because an unwind across `extern "C"` aborts the Grid.
#![allow(unsafe_code)]

use crate::gait::{SEGMENTS, joint_targets};
use std::sync::atomic::{AtomicU32, Ordering};

/// The interface version this Program was written against (`TGL_ABI_VERSION`).
pub const ABI_VERSION: u32 = 10;

/// `TglLibraryInfo`.
#[repr(C)]
pub struct LibraryInfo {
    pub creature_count: u32,
    pub nominal_dt_seconds: f32,
}

/// `TglCreatureDesc`.
#[repr(C)]
pub struct CreatureDesc {
    pub creature_id: u64,
    pub random_seed: u64,
    pub eyes: *const u8,
    pub ears: *const u8,
    pub eye_count: u32,
    pub ear_count: u32,
    pub irradiance_sample_count: u32,
    pub max_contact_count: u32,
    pub max_forward_speed: f32,
    pub max_turn_rate: f32,
    pub max_vocalisation_strength: f32,
    pub max_joint_angle: f32,
    pub max_joint_torque: f32,
    pub padding0: u32,
}

/// `TglRenderMaterial`.
#[repr(C)]
pub struct RenderMaterial {
    pub colour: [f32; 3],
    pub index_of_refraction: f32,
    pub emission: [f32; 3],
    pub transmission: f32,
}

/// `TglRenderTriangle`.
#[repr(C)]
pub struct RenderTriangle {
    pub vertices: [u32; 3],
    pub material: u32,
}

/// `TglRenderModel`.
#[repr(C)]
pub struct RenderModel {
    pub vertex_positions: *const f32,
    pub triangles: *const RenderTriangle,
    pub materials: *const RenderMaterial,
    pub vertex_count: u32,
    pub triangle_count: u32,
    pub material_count: u32,
    pub segment_count: u32,
    pub segment_spacing: f32,
    pub padding0: u32,
}

/// The first member of `TglSenses`, the only one this Program reads: ticks since the run began.
#[repr(C)]
pub struct SensesHead {
    pub tick: u64,
}

/// `TglActions`.
#[repr(C)]
pub struct Actions {
    pub desired_forward_speed: f32,
    pub desired_turn_rate: f32,
    pub vocalisation_strength: f32,
    pub joint_targets: [f32; SEGMENTS - 1],
}

/// `TglProgramVTable`.
#[repr(C)]
pub struct ProgramVTable {
    pub struct_size: u32,
    pub abi_version: u32,
    pub library_init: extern "C" fn(*const LibraryInfo),
    pub program_rez: extern "C" fn(*const CreatureDesc, *mut RenderModel) -> *mut Creature,
    pub program_tick: extern "C" fn(*mut Creature, *const SensesHead, *mut Actions),
    pub program_derez: extern "C" fn(*mut Creature),
    pub library_shutdown: extern "C" fn(),
}

// The layout of ABI version 10, from tron-grid-lite's libs/program-abi/abi_layout.txt, checked
// when this crate compiles; the tests compare every member with that file when it is present.
const _: () = {
    use std::mem::{offset_of, size_of};
    assert!(size_of::<LibraryInfo>() == 8 && offset_of!(LibraryInfo, nominal_dt_seconds) == 4);
    assert!(size_of::<CreatureDesc>() == 72 && offset_of!(CreatureDesc, max_joint_angle) == 60);
    assert!(size_of::<RenderMaterial>() == 32 && size_of::<RenderTriangle>() == 16);
    assert!(size_of::<RenderModel>() == 48 && offset_of!(RenderModel, segment_spacing) == 40);
    assert!(size_of::<Actions>() == 40 && offset_of!(Actions, joint_targets) == 12);
    assert!(size_of::<ProgramVTable>() == 48 && offset_of!(ProgramVTable, library_shutdown) == 40);
};

/// One creature's state: the bound on its joints, from its descriptor.
pub struct Creature {
    max_joint_angle: f64,
}

/// The tick length, set once by `library_init`, as `f32` bits.
static DT_BITS: AtomicU32 = AtomicU32::new(0);

/// The body: one small tetrahedron per segment, nose forward along -Z, mirror-dark with a neon back.
static VERTICES: [f32; 12] = [0.0, 0.01, -0.06, -0.03, 0.0, 0.03, 0.03, 0.0, 0.03, 0.0, 0.04, 0.03];
static TRIANGLES: [RenderTriangle; 4] = [
    RenderTriangle {
        vertices: [0, 2, 1],
        material: 0,
    },
    RenderTriangle {
        vertices: [0, 1, 3],
        material: 0,
    },
    RenderTriangle {
        vertices: [0, 3, 2],
        material: 0,
    },
    RenderTriangle {
        vertices: [1, 2, 3],
        material: 1,
    },
];
static MATERIALS: [RenderMaterial; 2] = [
    RenderMaterial {
        colour: [0.02, 0.02, 0.03],
        index_of_refraction: 1.5,
        emission: [0.0; 3],
        transmission: 0.0,
    },
    RenderMaterial {
        colour: [0.05, 0.05, 0.05],
        index_of_refraction: 1.5,
        emission: [0.1, 2.0, 3.0],
        transmission: 0.0,
    },
];
/// Metres between consecutive segments' origins: the tetrahedron's depth and a small gap.
pub const SEGMENT_SPACING: f32 = 0.1;

extern "C" fn library_init(info: *const LibraryInfo) {
    // SAFETY: the Grid passes a valid TglLibraryInfo for the duration of the call, or, defensively, null.
    if let Some(info) = unsafe { info.as_ref() } {
        DT_BITS.store(info.nominal_dt_seconds.to_bits(), Ordering::Relaxed);
    }
}

extern "C" fn program_rez(desc: *const CreatureDesc, model: *mut RenderModel) -> *mut Creature {
    // SAFETY: both pointers are valid for the call (the descriptor borrowed, the model the Grid's
    // zeroed struct for this Program to fill); null is refused rather than followed.
    let (Some(desc), Some(model)) = (unsafe { desc.as_ref() }, unsafe { model.as_mut() }) else {
        return std::ptr::null_mut();
    };
    model.vertex_positions = VERTICES.as_ptr();
    model.triangles = TRIANGLES.as_ptr();
    model.materials = MATERIALS.as_ptr();
    model.vertex_count = (VERTICES.len() / 3) as u32;
    model.triangle_count = TRIANGLES.len() as u32;
    model.material_count = MATERIALS.len() as u32;
    model.segment_count = SEGMENTS as u32;
    model.segment_spacing = SEGMENT_SPACING;
    Box::into_raw(Box::new(Creature {
        max_joint_angle: f64::from(desc.max_joint_angle),
    }))
}

extern "C" fn program_tick(creature: *mut Creature, senses: *const SensesHead, actions: *mut Actions) {
    // SAFETY: the creature is a handle this library returned from program_rez and has not freed;
    // the senses and actions are valid for the call. A null is ignored rather than followed.
    let (Some(creature), Some(senses), Some(actions)) = (unsafe { creature.as_ref() }, unsafe { senses.as_ref() }, unsafe { actions.as_mut() }) else {
        return;
    };
    let seconds = senses.tick as f64 * f64::from(f32::from_bits(DT_BITS.load(Ordering::Relaxed)));
    actions.desired_forward_speed = 0.0;
    actions.desired_turn_rate = 0.0;
    actions.vocalisation_strength = 0.0;
    for (slot, target) in actions.joint_targets.iter_mut().zip(joint_targets(seconds, creature.max_joint_angle)) {
        *slot = target as f32;
    }
}

extern "C" fn program_derez(creature: *mut Creature) {
    if !creature.is_null() {
        // SAFETY: a non-null handle is one program_rez made with Box::into_raw, freed here once.
        drop(unsafe { Box::from_raw(creature) });
    }
}

extern "C" fn library_shutdown() {}

static VTABLE: ProgramVTable = ProgramVTable {
    struct_size: std::mem::size_of::<ProgramVTable>() as u32,
    abi_version: ABI_VERSION,
    library_init,
    program_rez,
    program_tick,
    program_derez,
    library_shutdown,
};

/// The one symbol the Grid resolves. Null for any other ABI version, so a mismatch fails loudly.
#[unsafe(no_mangle)]
pub extern "C" fn tglGetProgramVTable(abi_version: u32) -> *const ProgramVTable {
    if abi_version == ABI_VERSION { &VTABLE } else { std::ptr::null() }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem::{offset_of, size_of};

    #[test]
    fn the_layout_matches_the_grids_published_one() {
        let path = concat!(env!("CARGO_MANIFEST_DIR"), "/../../external/tron-grid-lite/libs/program-abi/abi_layout.txt");
        let Ok(text) = std::fs::read_to_string(path) else {
            eprintln!("{path} is absent (submodules not fetched); the compile-time assertions above still hold");
            return;
        };
        let ours: Vec<(&str, &str, usize, usize)> = vec![
            ("TglLibraryInfo", "creature_count", offset_of!(LibraryInfo, creature_count), 4),
            ("TglLibraryInfo", "nominal_dt_seconds", offset_of!(LibraryInfo, nominal_dt_seconds), 4),
            ("TglCreatureDesc", "creature_id", offset_of!(CreatureDesc, creature_id), 8),
            ("TglCreatureDesc", "random_seed", offset_of!(CreatureDesc, random_seed), 8),
            ("TglCreatureDesc", "eyes", offset_of!(CreatureDesc, eyes), 8),
            ("TglCreatureDesc", "ears", offset_of!(CreatureDesc, ears), 8),
            ("TglCreatureDesc", "eye_count", offset_of!(CreatureDesc, eye_count), 4),
            ("TglCreatureDesc", "ear_count", offset_of!(CreatureDesc, ear_count), 4),
            ("TglCreatureDesc", "irradiance_sample_count", offset_of!(CreatureDesc, irradiance_sample_count), 4),
            ("TglCreatureDesc", "max_contact_count", offset_of!(CreatureDesc, max_contact_count), 4),
            ("TglCreatureDesc", "max_forward_speed", offset_of!(CreatureDesc, max_forward_speed), 4),
            ("TglCreatureDesc", "max_turn_rate", offset_of!(CreatureDesc, max_turn_rate), 4),
            ("TglCreatureDesc", "max_vocalisation_strength", offset_of!(CreatureDesc, max_vocalisation_strength), 4),
            ("TglCreatureDesc", "max_joint_angle", offset_of!(CreatureDesc, max_joint_angle), 4),
            ("TglCreatureDesc", "max_joint_torque", offset_of!(CreatureDesc, max_joint_torque), 4),
            ("TglCreatureDesc", "padding0", offset_of!(CreatureDesc, padding0), 4),
            ("TglRenderMaterial", "colour", offset_of!(RenderMaterial, colour), 12),
            ("TglRenderMaterial", "index_of_refraction", offset_of!(RenderMaterial, index_of_refraction), 4),
            ("TglRenderMaterial", "emission", offset_of!(RenderMaterial, emission), 12),
            ("TglRenderMaterial", "transmission", offset_of!(RenderMaterial, transmission), 4),
            ("TglRenderTriangle", "vertices", offset_of!(RenderTriangle, vertices), 12),
            ("TglRenderTriangle", "material", offset_of!(RenderTriangle, material), 4),
            ("TglRenderModel", "vertex_positions", offset_of!(RenderModel, vertex_positions), 8),
            ("TglRenderModel", "triangles", offset_of!(RenderModel, triangles), 8),
            ("TglRenderModel", "materials", offset_of!(RenderModel, materials), 8),
            ("TglRenderModel", "vertex_count", offset_of!(RenderModel, vertex_count), 4),
            ("TglRenderModel", "triangle_count", offset_of!(RenderModel, triangle_count), 4),
            ("TglRenderModel", "material_count", offset_of!(RenderModel, material_count), 4),
            ("TglRenderModel", "segment_count", offset_of!(RenderModel, segment_count), 4),
            ("TglRenderModel", "segment_spacing", offset_of!(RenderModel, segment_spacing), 4),
            ("TglRenderModel", "padding0", offset_of!(RenderModel, padding0), 4),
            ("TglSenses", "tick", offset_of!(SensesHead, tick), 8),
            ("TglActions", "desired_forward_speed", offset_of!(Actions, desired_forward_speed), 4),
            ("TglActions", "desired_turn_rate", offset_of!(Actions, desired_turn_rate), 4),
            ("TglActions", "vocalisation_strength", offset_of!(Actions, vocalisation_strength), 4),
            ("TglActions", "joint_targets", offset_of!(Actions, joint_targets), 28),
            ("TglProgramVTable", "struct_size", offset_of!(ProgramVTable, struct_size), 4),
            ("TglProgramVTable", "abi_version", offset_of!(ProgramVTable, abi_version), 4),
            ("TglProgramVTable", "library_init", offset_of!(ProgramVTable, library_init), 8),
            ("TglProgramVTable", "program_rez", offset_of!(ProgramVTable, program_rez), 8),
            ("TglProgramVTable", "program_tick", offset_of!(ProgramVTable, program_tick), 8),
            ("TglProgramVTable", "program_derez", offset_of!(ProgramVTable, program_derez), 8),
            ("TglProgramVTable", "library_shutdown", offset_of!(ProgramVTable, library_shutdown), 8),
        ];
        assert!(
            text.lines().any(|line| line == format!("abi-version {ABI_VERSION}")),
            "the file is not ABI version {ABI_VERSION}"
        );
        for (strukt, member, offset, size) in ours {
            let line = format!("member {strukt} {member} {offset} {size}");
            assert!(text.lines().any(|l| l == line), "{line} is not in the published layout");
        }
        for (strukt, size) in [
            ("TglLibraryInfo", size_of::<LibraryInfo>()),
            ("TglCreatureDesc", size_of::<CreatureDesc>()),
            ("TglRenderMaterial", size_of::<RenderMaterial>()),
            ("TglRenderTriangle", size_of::<RenderTriangle>()),
            ("TglRenderModel", size_of::<RenderModel>()),
            ("TglActions", size_of::<Actions>()),
            ("TglProgramVTable", size_of::<ProgramVTable>()),
        ] {
            assert!(
                text.lines().any(|l| l.starts_with(&format!("struct {strukt} {size} "))),
                "{strukt} is not {size} bytes there"
            );
        }
    }

    #[test]
    fn the_vtable_answers_only_its_own_version() {
        assert!(tglGetProgramVTable(ABI_VERSION - 1).is_null());
        let vtable = unsafe { &*tglGetProgramVTable(ABI_VERSION) };
        assert_eq!((vtable.struct_size, vtable.abi_version), (48, ABI_VERSION));
    }

    #[test]
    fn a_creature_rezzes_ticks_and_derezzes() {
        library_init(&LibraryInfo {
            creature_count: 1,
            nominal_dt_seconds: 0.031_25,
        });
        let desc = CreatureDesc {
            creature_id: 1,
            random_seed: 2,
            eyes: std::ptr::null(),
            ears: std::ptr::null(),
            eye_count: 0,
            ear_count: 0,
            irradiance_sample_count: 0,
            max_contact_count: 0,
            max_forward_speed: 0.0,
            max_turn_rate: 0.0,
            max_vocalisation_strength: 0.0,
            max_joint_angle: 0.9,
            max_joint_torque: 1.0,
            padding0: 0,
        };
        let mut model: RenderModel = unsafe { std::mem::zeroed() };
        let creature = program_rez(&desc, &mut model);
        assert!(!creature.is_null());
        assert_eq!((model.segment_count, model.triangle_count, model.vertex_count, model.material_count), (8, 4, 4, 2));
        // A quarter period in (0.25 / 0.18 s, at 32 ticks a second), the head joint is near its peak.
        let mut actions = Actions {
            desired_forward_speed: 1.0,
            desired_turn_rate: 1.0,
            vocalisation_strength: 1.0,
            joint_targets: [9.0; 7],
        };
        program_tick(creature, &SensesHead { tick: 44 }, &mut actions);
        assert_eq!(
            (actions.desired_forward_speed, actions.desired_turn_rate, actions.vocalisation_strength),
            (0.0, 0.0, 0.0)
        );
        assert!(actions.joint_targets.iter().all(|t| t.abs() <= 0.3), "{:?}", actions.joint_targets);
        assert!(actions.joint_targets[0] > 0.29, "{:?}", actions.joint_targets);
        program_derez(creature);
    }

    #[test]
    fn every_face_points_outwards_and_has_area() {
        let v = |i: u32| [VERTICES[3 * i as usize], VERTICES[3 * i as usize + 1], VERTICES[3 * i as usize + 2]];
        let centroid: [f32; 3] = std::array::from_fn(|axis| (0..4).map(|i| v(i)[axis]).sum::<f32>() / 4.0);
        for triangle in &TRIANGLES {
            let [a, b, c] = triangle.vertices.map(v);
            let (u, w) = ([b[0] - a[0], b[1] - a[1], b[2] - a[2]], [c[0] - a[0], c[1] - a[1], c[2] - a[2]]);
            let normal = [u[1] * w[2] - u[2] * w[1], u[2] * w[0] - u[0] * w[2], u[0] * w[1] - u[1] * w[0]];
            let outward = (0..3).map(|axis| normal[axis] * (a[axis] - centroid[axis])).sum::<f32>();
            assert!(outward > 0.0, "{triangle:?} faces inwards", triangle = triangle.vertices);
        }
    }
}
