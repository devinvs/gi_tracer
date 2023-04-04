use crate::world::World;
use crate::camera::Camera;

use std::sync::Arc;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct RenderJob {
    pub img_width: usize,
    pub img_height: usize,
    pub num_samples: usize,

    pub camera: Arc<Camera>,
    pub world: Arc<World>,

    pub start: usize,
    pub count: usize,
}
