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
            death_msgs.write(FatalHitMessage { victim: damage.target, killer: damage.source});
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

#[cfg(test)]
mod tests {
    use super::*;

    fn build_app() -> App {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.add_systems(Update, (deal_healing, deal_damage).chain());
        app.add_message::<DamageMessage>();
        app.add_message::<HealMessage>();
        app.add_message::<FatalHitMessage>();
        app
    }

    fn test_armour_formula(damage: f32, rating: f32) -> f32 {
        damage * (1. / rating)
    }

    #[test]
    fn damage_test() {
        let mut app = build_app();
        let trgt = app.world_mut().spawn((
            Name::new("Target"),
            Health(10.),
        )).id();
        app.world_mut().write_message(DamageMessage {
            target: trgt,
            source: None,
            amount: 5.,
        });
        app.update();
        let health = app.world().get::<Health>(trgt).unwrap();
        assert_eq!(health.0, 5.);
    }

    #[test]
    fn fatal_hit_test() {
        let mut app = build_app();
        let trgt = app.world_mut().spawn((
            Name::new("Target"),
            Health(10.),
        )).id();
        app.world_mut().write_message(DamageMessage {
            target: trgt,
            source: None,
            amount: 10.,
        });
        app.update();
        let messages = app.world().resource::<Messages<FatalHitMessage>>();
        assert!(!messages.is_empty());
    }

    #[test]
    fn negative_damage_test() {
        let mut app = build_app();
        let trgt = app.world_mut().spawn((
            Name::new("Target"),
            Health(10.),
        )).id();
        app.world_mut().write_message(DamageMessage {
            target: trgt,
            source: None,
            amount: -5.,
        });
        app.update();
        let health = app.world().get::<Health>(trgt).unwrap();
        assert_eq!(health.0, 10.);
    }

    #[test]
    fn heal_test() {
        let mut app = build_app();
        let trgt = app.world_mut().spawn((
            Name::new("Target"),
            Health(10.),
        )).id();
        app.world_mut().write_message(HealMessage {
            target: trgt,
            source: None,
            amount: 5.,
        });
        app.update();
        let health = app.world().get::<Health>(trgt).unwrap();
        assert_eq!(health.0, 15.);
    }

    #[test]
    fn negative_heal_test() {
        let mut app = build_app();
        let trgt = app.world_mut().spawn((
            Name::new("Target"),
            Health(10.),
        )).id();
        app.world_mut().write_message(HealMessage {
            target: trgt,
            source: None,
            amount: -5.,
        });
        app.update();
        let health = app.world().get::<Health>(trgt).unwrap();
        assert_eq!(health.0, 10.);
    }

    #[test]
    fn overheal_test() {
        let mut app = build_app();
        let trgt = app.world_mut().spawn((
            Name::new("Target"),
            Health(8.),
            MaxHealth(10.)
        )).id();
        app.world_mut().write_message(HealMessage {
            target: trgt,
            source: None,
            amount: 4.,
        });
        app.update();
        let health = app.world().get::<Health>(trgt).unwrap();
        assert_eq!(health.0, 10.);
    }

    #[test]
    fn max_damage_test() {
        let mut app = build_app();
        let trgt = app.world_mut().spawn((
            Name::new("Target"),
            MaxDamage(5.),
            Health(10.),
        )).id();
        app.world_mut().write_message(DamageMessage {
            target: trgt,
            source: None,
            amount: 8.,
        });
        app.update();
        let health = app.world().get::<Health>(trgt).unwrap();
        assert_eq!(health.0, 5.);
    }

    #[test]
    fn over_min_damage_test() {
        let mut app = build_app();
        let trgt = app.world_mut().spawn((
            Name::new("Target"),
            MinDamage(5.),
            Health(10.),
        )).id();
        app.world_mut().write_message(DamageMessage {
            target: trgt,
            source: None,
            amount: 8.,
        });
        app.update();
        let health = app.world().get::<Health>(trgt).unwrap();
        assert_eq!(health.0, 2.);
    }

    #[test]
    fn under_min_damage_test() {
        let mut app = build_app();
        let trgt = app.world_mut().spawn((
            Name::new("Target"),
            MinDamage(5.),
            Health(10.),
        )).id();
        app.world_mut().write_message(DamageMessage {
            target: trgt,
            source: None,
            amount: 3.,
        });
        app.update();
        let health = app.world().get::<Health>(trgt).unwrap();
        assert_eq!(health.0, 10.);
    }

    #[test]
    fn armour_test() {
        let mut app = build_app();
        let trgt = app.world_mut().spawn((
            Name::new("Target"),
            Armour {
                rating: 2.,
                formula: test_armour_formula // 1/rating 
            },
            Health(10.),
        )).id();
        app.world_mut().write_message(DamageMessage {
            target: trgt,
            source: None,
            amount: 8., // Should get halved
        });
        app.update();
        let health = app.world().get::<Health>(trgt).unwrap();
        assert_eq!(health.0, 6.);
    }
}
