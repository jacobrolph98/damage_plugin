use bevy::prelude::*;

#[derive(Component)]
pub struct Health(pub f32);

#[derive(Component)]
pub struct MaxHealth(pub f32);

#[derive(Component)]
pub struct DamageMultiplier(pub f32);

#[derive(Component)]
pub struct HealMultiplier(pub f32);

#[derive(Component)]
pub struct MaxDamage(pub f32);

#[derive(Component)]
pub struct MinDamage(pub f32);

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