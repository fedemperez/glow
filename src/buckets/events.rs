use legion::Entity;
use nalgebra::Vector3;
use super::coords::BucketCoords;

#[derive(Clone, Debug)]
pub enum EntityEvent {
    Appear {
        entity: Entity,
    },
    Disappear {
        id: u32,
    },
    Move {
        id: u32,
        from: Vector3<f32>,
        to: Vector3<f32>,
    },
    MoveAway {
        id: u32,
        to: BucketCoords,
    },
    MoveInto {
        entity: Entity,
        id: u32,
        old: BucketCoords,
        from: Vector3<f32>,
        to: Vector3<f32>,
    },
    Rotate {
        id: u32,
        yaw: f32,
        pitch: f32,
    },
    RotateHead {
        id: u32,
        yaw: f32,
        pitch: f32,
    }
}