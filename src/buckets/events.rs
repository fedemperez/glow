use legion::Entity;
use nalgebra::Vector3;
use super::coords::BucketCoords;

#[derive(Clone, Debug)]
pub struct EntityEvent {
    pub id: u32,
    pub data: EntityEventData,
}

#[derive(Clone, Debug)]
pub enum EntityEventData {
    Appear {
        entity: Entity,
    },
    Disappear,
    Move {
        delta: Vector3<f64>,
    },
    MoveAway {
        to: BucketCoords,
    },
    MoveInto {
        entity: Entity,
        from: BucketCoords,
    },
    MoveRotate {
        delta: Vector3<f64>,
        yaw: f32,
        pitch: f32,
    },
    Rotate {
        yaw: f32,
        pitch: f32,
    },
    RotateHead {
        yaw: f32,
    }
}