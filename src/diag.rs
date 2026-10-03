//! TEMP(诊断): 位移/动画流畅度排查。诊断完整个模块删除。

use bevy::prelude::*;

use crate::core::hero::commands::move_to_cell::MoveToCell;
use crate::core::map::components::cell_coord::CellCoord;
use crate::core::time::components::world_driver::WorldDriver;
use crate::frontend::display::motion::components::presentation_position::PresentationPosition;
use crate::frontend::display::sprite_animation::components::anim_state::AnimState;

#[derive(Resource, Default)]
pub struct DiagFrame(pub u32);

pub fn diag_frame_tick(mut diag: ResMut<DiagFrame>) {
    diag.0 += 1;
}

pub fn autodrive(
    diag: Res<DiagFrame>,
    mut move_commands: MessageWriter<MoveToCell>,
    mut sent: Local<bool>,
) {
    if *sent || diag.0 < 5 {
        return;
    }
    *sent = true;
    println!("[DIAG] f{} AUTODRIVE MoveToCell(45,10)", diag.0);
    move_commands.write(MoveToCell(CellCoord::new(45, 10)));
}

/// 全生物全景: 位置/格子/动画, 每帧每生物一行 (H=主角, M=怪)。
pub fn creature_probe(
    diag: Res<DiagFrame>,
    time: Res<Time>,
    mut creatures: Query<(
        &CellCoord,
        &PresentationPosition,
        &AnimState,
        Has<WorldDriver>,
    )>,
) {
    for (cell, presentation, state, is_driver) in &mut creatures {
        println!(
            "[DIAG] f{} PROBE {} pos={:.4} cell=({},{}) anim={:?} dt={:.1}ms",
            diag.0,
            if is_driver { 'H' } else { 'M' },
            presentation.x,
            cell.x,
            cell.y,
            state.anim,
            time.delta_secs() * 1000.0
        );
    }
}
