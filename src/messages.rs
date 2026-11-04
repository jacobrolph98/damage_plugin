use bevy::prelude::*;

use crate::types::*;

#[derive(Message)]
pub struct DamageMessage {
    pub target: Entity,
    pub damage: Damage,
    pub source: Option<Entity>
}

#[derive(Message)]
pub struct HealMessage {
    pub target: Entity,
    pub amount: f32,
    pub source: Option<Entity>
}

#[derive(Message)]
pub struct FatalHitMessage {
    pub target: Entity,
    pub source: Option<Entity>
}