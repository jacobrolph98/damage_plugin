use bevy::prelude::*;

use crate::types::*;

#[derive(Message)]
pub struct InflictDamageMessage {
    pub target: Entity,
    pub damage: Damage,
    pub source: Option<Entity>
}

#[derive(Message)]
pub struct ReportDamageMessage {
    pub target: Entity,
    pub amount: f32,
    pub source: Option<Entity>,
}

#[derive(Message)]
pub struct InflictHealMessage {
    pub target: Entity,
    pub amount: f32,
    pub source: Option<Entity>
}

#[derive(Message)]
pub struct ReportHealMessage {
    pub target: Entity,
    pub amount: f32,
    pub source: Option<Entity>,
}

#[derive(Message)]
pub struct FatalHitMessage {
    pub target: Entity,
    pub source: Option<Entity>
}