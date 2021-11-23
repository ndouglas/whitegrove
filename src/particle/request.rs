use specs::*;

use crate::model::Position;
use crate::render::Renderable;

struct Request {
    position: Position,
    renderable: Renderable,
    lifetime: f32,
}
