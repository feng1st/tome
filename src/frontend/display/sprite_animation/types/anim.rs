#[derive(Clone)]
pub struct Anim {
    pub frames: Vec<usize>,
    pub fps: f32,
    pub looped: bool,
}
