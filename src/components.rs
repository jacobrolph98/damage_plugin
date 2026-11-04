use bevy::{platform::collections::HashMap, prelude::*};

/// Tracks remaining health of an entity
#[derive(Component)]
pub struct Health(pub f32);

/// Sets a cap to the maximum health an entity can have
#[derive(Component)]
pub struct MaxHealth(pub f32);

/// Modifies incoming damage corresponding to Some(usize) representing damage type index according to a formula
/// Formula returns new damage value, not a multiplier/additive 
#[derive(Component)]
pub struct DamageModifier(pub HashMap<Option<usize>, fn(f32) -> f32>);

/// Multiplies all incoming healing by this amount
#[derive(Component)]
pub struct HealMultiplier(pub f32);

/// Incoming damage exceeding this amount is reduced to this amount
#[derive(Component)]
pub struct MaxDamage(pub f32);

/// Incoming damage less than this amount is ignored
#[derive(Component)]
pub struct MinDamage(pub f32);

/// Provide a rating and formula to modify incoming damage
#[derive(Component)]
pub struct Armour {
    pub rating: f32,
    pub formula: fn(f32, f32) -> f32 // damage, rating
}

impl Armour {
    pub fn apply_armour(&self, damage: f32) -> f32 {
        (self.formula)(damage, self.rating)
    }
}