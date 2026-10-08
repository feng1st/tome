//! Figure attachment: creatures gain the `FigureIndex` handle of the
//! figure their identity binds to (monsters by kind, humanoids by race
//! and class); appearance resolves the handle through the figure
//! registry and attaches the renderable parts.

pub mod attach_appearance;
pub mod attach_monster_figure;
pub mod attach_race_figure;
