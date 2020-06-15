use amethyst::ecs::{Component, DenseVecStorage};

pub struct SimpleAnimation {
    pub start_sprite_index: i32,
    pub frames: i32,
    pub current_frame: i32,
    pub time_per_frame: f32,
    pub elapsed_time: f32,
}

impl SimpleAnimation {
    pub fn new(start_sprite_index: i32, frames: i32, time_per_frame: f32) -> SimpleAnimation {
        SimpleAnimation {
            start_sprite_index: start_sprite_index,
            frames: frames,
            current_frame: 0,
            time_per_frame: time_per_frame,
            elapsed_time: 0.0,
        }
    }
}

impl Component for SimpleAnimation {
    type Storage = DenseVecStorage<Self>;
}
