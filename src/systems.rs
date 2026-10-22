use bevy::prelude::*;

use crate::{components::*, messages::*};

pub fn deal_damage(
    mut damage_msgs: MessageReader<DamageMessage>,
    mut health_q: Query<(
        &mut Health, 
        Option<&MaxDamage>, 
        Option<&MinDamage>, 
        Option<&Armour>,
        Option<&DamageMultiplier>,
    )>,
    mut death_msgs: MessageWriter<FatalHitMessage>,
) {
    for damage in damage_msgs.read() {
        if damage.amount <= 0. { continue }
        let Ok((
            mut health, 
            has_max_dam, 
            has_min_dam, 
            has_armour, 
            has_mult
        )) = health_q.get_mut(damage.target) else { continue };
        let mut dealt_damage = damage.amount;
        if let Some(multiplier) = has_mult {
            dealt_damage *= multiplier.0;
        }
        if let Some(armour) = has_armour {
            dealt_damage = armour.apply_armour(dealt_damage);
        }
        if let Some(minimum) = has_min_dam {
            if dealt_damage < minimum.0 {
                dealt_damage *= 0.;
            }
        }
        if let Some(maximum) = has_max_dam {
            if dealt_damage > maximum.0 {
                dealt_damage = maximum.0;
            }
        }        
        let dealt_damage = if dealt_damage >= health.0 { 
            death_msgs.write(FatalHitMessage { target: damage.target, source: damage.source});
            health.0
         } else { 
            dealt_damage 
        };
        health.0 -= dealt_damage;
    }
}

pub fn deal_healing(
    mut heal_msgs: MessageReader<HealMessage>,
    mut health_q: Query<(&mut Health, Option<&MaxHealth>, Option<&HealMultiplier>)>,
) {
    for heal in heal_msgs.read() {
        if heal.amount <= 0. { continue }
        let Ok((mut health, has_max_health, has_mult)) = health_q.get_mut(heal.target) else { continue };
        let mut dealt_healing = heal.amount;
        if let Some(multiplier) = has_mult {
            dealt_healing *= multiplier.0;
        }
        if let Some(max_health) = has_max_health {
            if dealt_healing + health.0 > max_health.0 {
                health.0 = max_health.0;
                continue;
            }
        }
        health.0 += dealt_healing;
    }
}