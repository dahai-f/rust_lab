use toybox::ecs::*;

#[component]
pub struct Velocity {
    x: f32,
    y: f32,
    z: f32,
}

#[component]
pub struct Parent {
    pub parent: Entity,
}
