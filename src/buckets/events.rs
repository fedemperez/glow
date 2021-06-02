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
        from: Vector3<f32>,
        to: Vector3<f32>,
    },
    MoveAway {
        to: BucketCoords,
    },
    MoveInto {
        entity: Entity,
        old: BucketCoords,
        from: Vector3<f32>,
        to: Vector3<f32>,
    },
    Rotate {
        yaw: f32,
        pitch: f32,
    },
    RotateHead {
        yaw: f32,
    }
}