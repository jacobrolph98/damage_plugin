use bevy::prelude::*;

use crate::{messages::*, systems::*};

pub struct DamagePlugin;

impl Plugin for DamagePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_message::<InflictDamageMessage>()
            .add_message::<ReportDamageMessage>()
            .add_message::<InflictHealMessage>()
            .add_message::<ReportHealMessage>()
            .add_message::<FatalHitMessage>()
            .add_systems(FixedUpdate, (deal_healing, deal_damage).chain())
            ;
    }
}