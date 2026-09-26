//! The animation clip value type.

/// One animation clip: a frame sequence into the entity's sprite sheet
/// and its playback rate. A pure value type — the template's anim map
/// (`AnimClips` component) stores these.
#[derive(Clone, Copy)]
pub struct AnimClip {
    pub frames: &'static [usize],
    pub fps: f32,
}
