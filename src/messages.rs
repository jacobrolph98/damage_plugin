use bevy::prelude::*;

#[derive(Message)]
pub struct DamageMessage {
    pub target: Entity,
    pub amount: f32,
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
    pub victim: Entity,
    pub killer: Option<Entity>
}