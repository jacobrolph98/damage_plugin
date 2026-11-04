use bevy::prelude::*;

use crate::{components::*, messages::*};

pub fn deal_damage(
    mut damage_r: MessageReader<InflictDamageMessage>,
    mut health_q: Query<(
        &mut Health, 
        Option<&MaxDamage>, 
        Option<&MinDamage>, 
        Option<&Armour>,
        Option<&DamageModifier>,
    )>,
    mut report_w: MessageWriter<ReportDamageMessage>,
    mut fatality_w: MessageWriter<FatalHitMessage>,
) {
    for damage_msg in damage_r.read() {
        let mut net_damage: f32 = 0.;
        for dam_instance in &damage_msg.damage.0 {
            if dam_instance.amount <= 0. { continue }
            let Ok((
                mut health, 
                has_max_dam, 
                has_min_dam, 
                has_armour, 
                has_mod
            )) = health_q.get_mut(damage_msg.target) else { continue };
            if health.0 <= 0. { continue };
            let mut dealt_damage = dam_instance.amount;
            if let Some(modifier) = has_mod {
                if let Some(modifier) = modifier.0.get(&dam_instance.dam_type) {
                    dealt_damage = (modifier)(dealt_damage);
                }
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
            if dealt_damage >= health.0 {
                net_damage += health.0;
                health.0 = 0.;
                fatality_w.write(FatalHitMessage { target: damage_msg.target, source: damage_msg.source});
                break
            }
            net_damage += dealt_damage;
            health.0 -= dealt_damage;
        }
        report_w.write(ReportDamageMessage {
            target: damage_msg.target,
            amount: net_damage,
            source: damage_msg.source
        });
    }
}

pub fn deal_healing(
    mut heal_r: MessageReader<InflictHealMessage>,
    mut report_w: MessageWriter<ReportHealMessage>,
    mut health_q: Query<(&mut Health, Option<&MaxHealth>, Option<&HealMultiplier>)>,
) {
    for heal in heal_r.read() {
        if heal.amount <= 0. { continue }
        let Ok((mut health, has_max_health, has_mult)) = health_q.get_mut(heal.target) else { continue };
        let mut dealt_healing = heal.amount;
        if let Some(multiplier) = has_mult {
            dealt_healing *= multiplier.0;
        }
        if let Some(max_health) = has_max_health {
            dealt_healing = if health.0 + dealt_healing > max_health.0 {
                max_health.0 - health.0
            } else { dealt_healing };
        }
        health.0 += dealt_healing;
        report_w.write(ReportHealMessage { 
            target: heal.target, 
            amount: dealt_healing, 
            source: heal.source
        });
    }
}